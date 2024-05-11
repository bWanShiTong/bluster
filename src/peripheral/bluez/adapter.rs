use dbus::{
    arg::{messageitem::MessageItem, RefArg, Variant},
    Path,
};
use std::{collections::HashMap, sync::Arc};

use super::{
    connection::Connection,
    constants::{
        ADAPTER_IFACE, DBUS_OBJECTMANAGER_IFACE, DBUS_PROPERTIES_IFACE,
        LE_ADVERTISING_MANAGER_IFACE,
    },
};
use crate::Error;

#[derive(Debug, Clone)]
pub struct Adapter {
    pub object_path: Path<'static>,
    connection: Arc<Connection>,
}

type ManagedObjectsProps =
    HashMap<Path<'static>, HashMap<String, HashMap<String, Variant<Box<dyn RefArg>>>>>;

impl Adapter {
    async fn find_adapter(
        connection: &Arc<Connection>,
        name: Option<&str>,
    ) -> Result<Path<'static>, Error> {
        let path = "/".into();
        let proxy = connection.get_bluez_proxy(&path);

        let (props,): (ManagedObjectsProps,) = proxy
            .method_call(DBUS_OBJECTMANAGER_IFACE, "GetManagedObjects", ())
            .await?;

        let interfaces = props
            .into_iter()
            .filter(|(_path, props)| props.contains_key(LE_ADVERTISING_MANAGER_IFACE))
            .map(|(path, _props)| path)
            .collect::<Vec<_>>();

        let interface = if let Some(name) = name {
            interfaces.into_iter().find(|iface| iface.contains(name))
        } else {
            interfaces.first().cloned()
        };

        Ok(interface.expect("No interfaces found").to_owned())
    }

    #[allow(clippy::new_ret_no_self)]
    pub async fn new(connection: Arc<Connection>, interface: Option<&str>) -> Result<Self, Error> {
        Adapter::find_adapter(&connection, interface)
            .await
            .map(|object_path| Adapter {
                object_path,
                connection,
            })
    }

    pub async fn powered(self: &Self, on: bool) -> Result<(), Error> {
        let proxy = self.connection.get_bluez_proxy(&self.object_path);
        proxy
            .method_call(
                DBUS_PROPERTIES_IFACE,
                "Set",
                (
                    ADAPTER_IFACE,
                    "Powered",
                    MessageItem::Variant(Box::new(on.into())),
                ),
            )
            .await?;
        Ok(())
    }

    pub async fn is_powered(self: &Self) -> Result<bool, Error> {
        let proxy = self.connection.get_bluez_proxy(&self.object_path);
        let (powered,): (Variant<bool>,) = proxy
            .method_call(DBUS_PROPERTIES_IFACE, "Get", (ADAPTER_IFACE, "Powered"))
            .await?;
        Ok(powered.0)
    }

    pub async fn get_alias(self: &Self) -> Result<String, Error> {
        let proxy = self.connection.get_bluez_proxy(&self.object_path);
        let (alias,): (Variant<String>,) = proxy
            .method_call(DBUS_PROPERTIES_IFACE, "Get", (ADAPTER_IFACE, "Alias"))
            .await?;
        Ok(alias.0)
    }

    pub async fn set_alias(self: &Self, alias: &str) -> Result<(), Error> {
        let proxy = self.connection.get_bluez_proxy(&self.object_path);
        proxy
            .method_call(
                DBUS_PROPERTIES_IFACE,
                "Set",
                (
                    ADAPTER_IFACE,
                    "Alias",
                    MessageItem::Variant(Box::new(String::from(alias).into())),
                ),
            )
            .await?;
        Ok(())
    }
}

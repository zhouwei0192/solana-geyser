use geyser::Geyser;
use geyser_plugin_interface::GeyserPlugin;



pub mod geyser;
pub mod geyser_plugin_interface;
pub mod message;


#[no_mangle]
#[allow(improper_ctypes_definitions)]
/// # Safety
///
/// This function returns the Plugin pointer as trait GeyserPlugin.
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    let plugin = Geyser::default();
    let plugin: Box<dyn GeyserPlugin> = Box::new(plugin);
    Box::into_raw(plugin)
}
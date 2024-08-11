use std::collections::HashSet;
use std::ffi::CStr;
use std::os::raw::c_void;
use tracing::{debug, error, info, trace, warn};
use vulkanalia::{vk, Device, Entry, Instance, Version};
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::vk::{DebugUtilsMessengerEXT, DeviceV1_0, EntryV1_0, ExtDebugUtilsExtension, HasBuilder, InstanceV1_0, PhysicalDevice, Queue};
use vulkanalia::window as vk_window;
use winit::raw_window_handle::HasWindowHandle;

const PORTABILITY_MACOS_VERSION: Version = Version::new(1, 3, 216);
const VALIDATION_ENABLED: bool = cfg!(debug_assertions);
const VALIDATION_LAYER: vk::ExtensionName =
    vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");

pub (in crate::renderer) struct VulkanRenderer {
    entry: Entry,
    instance: Instance,
    physical_device: PhysicalDevice,
    logical_device: Device,
    graphics_queue: Queue,
    messenger: Option<DebugUtilsMessengerEXT>
}

impl VulkanRenderer {
    pub (in crate::renderer) fn new(window_handle_provider: &dyn HasWindowHandle) -> Self {
        unsafe {
            initialize_vulkan(window_handle_provider)
        }
    }

    pub (in crate::renderer) fn resume(&self) {
        trace!("Resume");
    }

    pub (in crate::renderer) fn render(&self) {
        trace!("Render");
    }

    pub (in crate::renderer) fn destroy(&self) {
        trace!("Destroy");
        unsafe { self.logical_device.destroy_device(None) }
        unsafe { if VALIDATION_ENABLED { self.instance.destroy_debug_utils_messenger_ext(self.messenger.unwrap(), None); } }
        unsafe { self.instance.destroy_instance(None); }
    }
}

unsafe fn initialize_vulkan(
    window_handle_provider: &dyn HasWindowHandle,
) -> VulkanRenderer {
    unsafe {
        let loader = LibloadingLoader::new(LIBRARY).unwrap();
        let entry = Entry::new(loader).unwrap();

        let application_info = vk::ApplicationInfo::builder()
            .application_name(b"Vulkan Tutorial\0")
            .application_version(vk::make_version(1, 0, 0))
            .engine_name(b"No Engine\0")
            .engine_version(vk::make_version(1, 0, 0))
            .api_version(vk::make_version(1, 0, 0));

        let available_layers = entry
            .enumerate_instance_layer_properties().unwrap()
            .iter()
            .map(|l| l.layer_name)
            .collect::<HashSet<_>>();

        if VALIDATION_ENABLED && !available_layers.contains(&VALIDATION_LAYER) {
            return panic!("Validation layer requested but not supported.");
        }

        let layers = if VALIDATION_ENABLED {
            vec![VALIDATION_LAYER.as_ptr()]
        } else {
            Vec::new()
        };

        let mut extensions = vk_window::get_required_instance_extensions(&window_handle_provider)
            .iter()
            .map(|e| e.as_ptr())
            .collect::<Vec<_>>();

        if VALIDATION_ENABLED {
            extensions.push(vk::EXT_DEBUG_UTILS_EXTENSION.name.as_ptr());
        }

        // Required by Vulkan SDK on macOS since 1.3.216.
        let flags = if cfg!(target_os = "macos") && entry.version().unwrap() >= PORTABILITY_MACOS_VERSION {
            trace!("Enabling extensions for macOS portability.");
            extensions.push(vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION.name.as_ptr());
            extensions.push(vk::KHR_PORTABILITY_ENUMERATION_EXTENSION.name.as_ptr());
            vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
        } else {
            vk::InstanceCreateFlags::empty()
        };

        let mut info = vk::InstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_layer_names(&layers)
            .enabled_extension_names(&extensions)
            .flags(flags);

        let mut debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
            .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::all())
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
            )
            .user_callback(Some(debug_callback));

        if VALIDATION_ENABLED {
            info = info.push_next(&mut debug_info);
        }

        let instance = entry.create_instance(&info, None).unwrap();
        trace!("{:?}", instance);

        let physical_device = pick_physical_device(&instance);
        let logical_device = create_logical_device(&entry, &instance, &physical_device);
        let graphics_queue = create_graphics_queue(&instance, &physical_device, &logical_device);

        if VALIDATION_ENABLED {
            let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
                .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::all())
                .message_type(
                    vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                        | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                        | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
                )
                .user_callback(Some(debug_callback));

            let messenger = instance.create_debug_utils_messenger_ext(&debug_info, None);

            VulkanRenderer {
                entry,
                instance,
                physical_device,
                logical_device,
                graphics_queue,
                messenger: Some(messenger.unwrap())
            }
        } else {
            VulkanRenderer {
                entry, instance,
                physical_device,
                logical_device,
                graphics_queue,
                messenger: None
            }
        }
    }
}

unsafe fn create_graphics_queue(
    instance: &Instance,
    physical_device: &PhysicalDevice,
    logical_device: &Device
) -> Queue {
    let indices = QueueFamilyIndices::get(instance, *physical_device);
    logical_device.get_device_queue(indices.graphics, 0)
}

unsafe fn create_logical_device(
    entry: &Entry,
    instance: &Instance,
    physical_device: &PhysicalDevice
) -> Device {
    let indices = QueueFamilyIndices::get(instance, *physical_device);

    let queue_priorities = &[1.0];
    let queue_info = vk::DeviceQueueCreateInfo::builder()
        .queue_family_index(indices.graphics)
        .queue_priorities(queue_priorities);

    let layers = if VALIDATION_ENABLED {
        vec![VALIDATION_LAYER.as_ptr()]
    } else {
        vec![]
    };

    let mut extensions = vec![];

    // Required by Vulkan SDK on macOS since 1.3.216.
    if cfg!(target_os = "macos") && entry.version().unwrap() >= PORTABILITY_MACOS_VERSION {
        extensions.push(vk::KHR_PORTABILITY_SUBSET_EXTENSION.name.as_ptr());
    }

    let features = vk::PhysicalDeviceFeatures::builder();

    let queue_infos = &[queue_info];
    let info = vk::DeviceCreateInfo::builder()
        .queue_create_infos(queue_infos)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&extensions)
        .enabled_features(&features);

    instance.create_device(*physical_device, &info, None).unwrap()
}

#[derive(Copy, Clone, Debug)]
struct QueueFamilyIndices {
    graphics: u32,
}

impl QueueFamilyIndices {
    unsafe fn get(
        instance: &Instance,
        physical_device: vk::PhysicalDevice,
    ) -> Self {
        let properties = instance
            .get_physical_device_queue_family_properties(physical_device);

        let graphics = properties
            .iter()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .map(|i| i as u32);

        if let Some(graphics) = graphics {
            Self { graphics }
        } else {
            panic!("Missing required queue families.")
        }
    }
}

unsafe fn pick_physical_device(instance: &Instance) -> vk::PhysicalDevice {
    for physical_device in instance.enumerate_physical_devices().unwrap() {
        let properties = instance.get_physical_device_properties(physical_device);

        if check_physical_device(instance, physical_device) {
            warn!("Skipping physical device (`{}`)", properties.device_name);
        } else {
            info!("Selected physical device (`{}`).", properties.device_name);
            return physical_device;
        }
    }
    panic!("lol")
}

unsafe fn check_physical_device(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
) -> bool {
    let properties = instance.get_physical_device_properties(physical_device);
    if properties.device_type != vk::PhysicalDeviceType::DISCRETE_GPU {
        //return Err(anyhow!(SuitabilityError("Only discrete GPUs are supported.")));
        return false
    }

    let features = instance.get_physical_device_features(physical_device);
    if features.geometry_shader != vk::TRUE {
        //return Err(anyhow!(SuitabilityError("Missing geometry shader support.")));
        return false
    }

    true
}

extern "system" fn debug_callback(
    severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    type_: vk::DebugUtilsMessageTypeFlagsEXT,
    data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _: *mut c_void,
) -> vk::Bool32 {
    let data = unsafe { *data };
    let message = unsafe { CStr::from_ptr(data.message) }.to_string_lossy();

    if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::ERROR {
        error!("({:?}) {}", type_, message);
    } else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::WARNING {
        warn!("({:?}) {}", type_, message);
    } else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::INFO {
        debug!("({:?}) {}", type_, message);
    } else {
        trace!("({:?}) {}", type_, message);
    }

    vk::FALSE
}
// ⚠️ GENERATED CODE ⚠️ - this entire file was generated by the `roc glue` CLI command

#![allow(unused_unsafe)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
#![allow(clippy::unit_cmp)]
#![allow(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::unused_unit)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::let_and_return)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::clone_on_copy)]

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "x86",
    target_arch = "x86_64"
))]
#[derive(Clone, Copy, Eq, Ord, Hash, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum discriminant_U1 {
    FromJob = 0,
    FromProjectSource = 1,
}

impl core::fmt::Debug for discriminant_U1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::FromJob => f.write_str("discriminant_U1::FromJob"),
            Self::FromProjectSource => f.write_str("discriminant_U1::FromProjectSource"),
        }
    }
}

#[cfg(any(target_arch = "arm", target_arch = "wasm32", target_arch = "x86"))]
#[repr(C)]
pub union U1 {
    FromJob: core::mem::ManuallyDrop<U1_FromJob>,
    FromProjectSource: core::mem::ManuallyDrop<roc_std::RocList<roc_std::RocStr>>,
    _sizer: [u8; 20],
}

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "x86",
    target_arch = "x86_64"
))]
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Rbt {
    pub default: Job,
}

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "x86",
    target_arch = "x86_64"
))]
#[repr(transparent)]
pub struct Job {
    pointer: *mut union_Job,
}

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "x86",
    target_arch = "x86_64"
))]
#[repr(C)]
union union_Job {
    Job: core::mem::ManuallyDrop<Job_Job>,
}

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "x86",
    target_arch = "x86_64"
))]
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd)]
#[repr(transparent)]
struct Job_Job {
    pub f0: R1,
}

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "x86",
    target_arch = "x86_64"
))]
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd)]
#[repr(C)]
pub struct R1 {
    pub command: Command,
    pub env: roc_std::RocDict<roc_std::RocStr, roc_std::RocStr>,
    pub inputs: roc_std::RocList<U1>,
    pub outputs: roc_std::RocList<roc_std::RocStr>,
}

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "x86",
    target_arch = "x86_64"
))]
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd)]
#[repr(C)]
struct U1_FromJob {
    pub f0: Job,
    pub f1: roc_std::RocList<roc_std::RocStr>,
}

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "x86",
    target_arch = "x86_64"
))]
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Command {
    pub args: roc_std::RocList<roc_std::RocStr>,
    pub tool: Tool,
}

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "x86",
    target_arch = "x86_64"
))]
#[repr(transparent)]
#[derive(Clone, Default, Eq, Ord, Hash, PartialEq, PartialOrd)]
pub struct Tool {
    f0: SystemToolPayload,
}

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "x86",
    target_arch = "x86_64"
))]
#[derive(Clone, Debug, Default, Eq, Ord, Hash, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SystemToolPayload {
    pub name: roc_std::RocStr,
}

#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[repr(C)]
pub union U1 {
    FromJob: core::mem::ManuallyDrop<U1_FromJob>,
    FromProjectSource: core::mem::ManuallyDrop<roc_std::RocList<roc_std::RocStr>>,
    _sizer: [u8; 40],
}

impl U1 {
    #[cfg(any(target_arch = "arm", target_arch = "wasm32", target_arch = "x86"))]
    /// Returns which variant this tag union holds. Note that this never includes a payload!
    pub fn discriminant(&self) -> discriminant_U1 {
        unsafe {
            let bytes = core::mem::transmute::<&Self, &[u8; core::mem::size_of::<Self>()]>(self);

            core::mem::transmute::<u8, discriminant_U1>(*bytes.as_ptr().add(16))
        }
    }

    #[cfg(any(target_arch = "arm", target_arch = "wasm32", target_arch = "x86"))]
    /// Internal helper
    fn set_discriminant(&mut self, discriminant: discriminant_U1) {
        let discriminant_ptr: *mut discriminant_U1 = (self as *mut U1).cast();

        unsafe {
            *(discriminant_ptr.add(16)) = discriminant;
        }
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Construct a tag named `FromJob`, with the appropriate payload
    pub fn FromJob(arg0: Job, arg1: roc_std::RocList<roc_std::RocStr>) -> Self {
        let mut answer = Self {
            FromJob: core::mem::ManuallyDrop::new(U1_FromJob { f0: arg0, f1: arg1 }),
        };

        answer.set_discriminant(discriminant_U1::FromJob);

        answer
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Unsafely assume the given `U1` has a `.discriminant()` of `FromJob` and convert it to `FromJob`'s payload.
    /// (Always examine `.discriminant()` first to make sure this is the correct variant!)
    /// Panics in debug builds if the `.discriminant()` doesn't return `FromJob`.
    pub unsafe fn into_FromJob(mut self) -> (Job, roc_std::RocList<roc_std::RocStr>) {
        debug_assert_eq!(self.discriminant(), discriminant_U1::FromJob);
        let payload = {
            let mut uninitialized = core::mem::MaybeUninit::uninit();
            let swapped = unsafe {
                core::mem::replace(
                    &mut self.FromJob,
                    core::mem::ManuallyDrop::new(uninitialized.assume_init()),
                )
            };

            core::mem::forget(self);

            core::mem::ManuallyDrop::into_inner(swapped)
        };

        (payload.f0, payload.f1)
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Unsafely assume the given `U1` has a `.discriminant()` of `FromJob` and return its payload.
    /// (Always examine `.discriminant()` first to make sure this is the correct variant!)
    /// Panics in debug builds if the `.discriminant()` doesn't return `FromJob`.
    pub unsafe fn as_FromJob(&self) -> (&Job, &roc_std::RocList<roc_std::RocStr>) {
        debug_assert_eq!(self.discriminant(), discriminant_U1::FromJob);
        let payload = &self.FromJob;

        (&payload.f0, &payload.f1)
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Construct a tag named `FromProjectSource`, with the appropriate payload
    pub fn FromProjectSource(arg: roc_std::RocList<roc_std::RocStr>) -> Self {
        let mut answer = Self {
            FromProjectSource: core::mem::ManuallyDrop::new(arg),
        };

        answer.set_discriminant(discriminant_U1::FromProjectSource);

        answer
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Unsafely assume the given `U1` has a `.discriminant()` of `FromProjectSource` and convert it to `FromProjectSource`'s payload.
    /// (Always examine `.discriminant()` first to make sure this is the correct variant!)
    /// Panics in debug builds if the `.discriminant()` doesn't return `FromProjectSource`.
    pub unsafe fn into_FromProjectSource(mut self) -> roc_std::RocList<roc_std::RocStr> {
        debug_assert_eq!(self.discriminant(), discriminant_U1::FromProjectSource);
        let payload = {
            let mut uninitialized = core::mem::MaybeUninit::uninit();
            let swapped = unsafe {
                core::mem::replace(
                    &mut self.FromProjectSource,
                    core::mem::ManuallyDrop::new(uninitialized.assume_init()),
                )
            };

            core::mem::forget(self);

            core::mem::ManuallyDrop::into_inner(swapped)
        };

        payload
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Unsafely assume the given `U1` has a `.discriminant()` of `FromProjectSource` and return its payload.
    /// (Always examine `.discriminant()` first to make sure this is the correct variant!)
    /// Panics in debug builds if the `.discriminant()` doesn't return `FromProjectSource`.
    pub unsafe fn as_FromProjectSource(&self) -> &roc_std::RocList<roc_std::RocStr> {
        debug_assert_eq!(self.discriminant(), discriminant_U1::FromProjectSource);
        let payload = &self.FromProjectSource;

        &payload
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    /// Returns which variant this tag union holds. Note that this never includes a payload!
    pub fn discriminant(&self) -> discriminant_U1 {
        unsafe {
            let bytes = core::mem::transmute::<&Self, &[u8; core::mem::size_of::<Self>()]>(self);

            core::mem::transmute::<u8, discriminant_U1>(*bytes.as_ptr().add(32))
        }
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    /// Internal helper
    fn set_discriminant(&mut self, discriminant: discriminant_U1) {
        let discriminant_ptr: *mut discriminant_U1 = (self as *mut U1).cast();

        unsafe {
            *(discriminant_ptr.add(32)) = discriminant;
        }
    }
}

impl Drop for U1 {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn drop(&mut self) {
        // Drop the payloads
        match self.discriminant() {
            discriminant_U1::FromJob => unsafe { core::mem::ManuallyDrop::drop(&mut self.FromJob) },
            discriminant_U1::FromProjectSource => unsafe {
                core::mem::ManuallyDrop::drop(&mut self.FromProjectSource)
            },
        }
    }
}

impl Eq for U1 {}

impl PartialEq for U1 {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn eq(&self, other: &Self) -> bool {
        if self.discriminant() != other.discriminant() {
            return false;
        }

        unsafe {
            match self.discriminant() {
                discriminant_U1::FromJob => self.FromJob == other.FromJob,
                discriminant_U1::FromProjectSource => {
                    self.FromProjectSource == other.FromProjectSource
                }
            }
        }
    }
}

impl PartialOrd for U1 {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        match self.discriminant().partial_cmp(&other.discriminant()) {
            Some(core::cmp::Ordering::Equal) => {}
            not_eq => return not_eq,
        }

        unsafe {
            match self.discriminant() {
                discriminant_U1::FromJob => self.FromJob.partial_cmp(&other.FromJob),
                discriminant_U1::FromProjectSource => {
                    self.FromProjectSource.partial_cmp(&other.FromProjectSource)
                }
            }
        }
    }
}

impl Ord for U1 {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        match self.discriminant().cmp(&other.discriminant()) {
            core::cmp::Ordering::Equal => {}
            not_eq => return not_eq,
        }

        unsafe {
            match self.discriminant() {
                discriminant_U1::FromJob => self.FromJob.cmp(&other.FromJob),
                discriminant_U1::FromProjectSource => {
                    self.FromProjectSource.cmp(&other.FromProjectSource)
                }
            }
        }
    }
}

impl Clone for U1 {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn clone(&self) -> Self {
        let mut answer = unsafe {
            match self.discriminant() {
                discriminant_U1::FromJob => Self {
                    FromJob: self.FromJob.clone(),
                },
                discriminant_U1::FromProjectSource => Self {
                    FromProjectSource: self.FromProjectSource.clone(),
                },
            }
        };

        answer.set_discriminant(self.discriminant());

        answer
    }
}

impl core::hash::Hash for U1 {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self.discriminant() {
            discriminant_U1::FromJob => unsafe {
                discriminant_U1::FromJob.hash(state);
                self.FromJob.hash(state);
            },
            discriminant_U1::FromProjectSource => unsafe {
                discriminant_U1::FromProjectSource.hash(state);
                self.FromProjectSource.hash(state);
            },
        }
    }
}

impl core::fmt::Debug for U1 {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("U1::")?;

        unsafe {
            match self.discriminant() {
                discriminant_U1::FromJob => f
                    .debug_tuple("FromJob")
                    .field(&(&*self.FromJob).f0)
                    .field(&(&*self.FromJob).f1)
                    .finish(),
                discriminant_U1::FromProjectSource => f
                    .debug_tuple("FromProjectSource")
                    .field(&*self.FromProjectSource)
                    .finish(),
            }
        }
    }
}

impl Job {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn storage(&self) -> Option<&core::cell::Cell<roc_std::Storage>> {
        let untagged = self.pointer as *const core::cell::Cell<roc_std::Storage>;

        unsafe { Some(&*untagged.sub(1)) }
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// This is a single-tag union, so it has no alternatives
    /// to discriminate between. This method is only included for completeness.
    pub fn discriminant(&self) -> () {
        ()
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Construct a tag named `Job`, with the appropriate payload
    pub fn Job(arg0: R1) -> Self {
        let size = core::mem::size_of::<union_Job>();
        let align = core::mem::align_of::<union_Job>() as u32;

        unsafe {
            let ptr = roc_std::roc_alloc_refcounted::<union_Job>();

            *ptr = union_Job {
                Job: core::mem::ManuallyDrop::new(Job_Job { f0: arg0 }),
            };

            Self { pointer: ptr }
        }
    }

    #[cfg(any(target_arch = "arm", target_arch = "wasm32", target_arch = "x86"))]
    /// Since `Job` only has one tag (namely, `Job`),
    /// convert it to `Job`'s payload.
    pub fn into_Job(mut self) -> R1 {
        let payload = {
            let ptr = (self.pointer as usize & !0b11) as *mut union_Job;
            let mut uninitialized = core::mem::MaybeUninit::uninit();
            let swapped = unsafe {
                core::mem::replace(
                    &mut (*ptr).Job,
                    core::mem::ManuallyDrop::new(uninitialized.assume_init()),
                )
            };

            core::mem::forget(self);

            core::mem::ManuallyDrop::into_inner(swapped)
        };

        payload.f0
    }

    #[cfg(any(target_arch = "arm", target_arch = "wasm32", target_arch = "x86"))]
    /// Since `Job` only has one tag (namely, `Job`),
    /// convert it to `Job`'s payload.
    pub fn as_Job(&self) -> &R1 {
        let payload = {
            let ptr = (self.pointer as usize & !0b11) as *mut union_Job;

            unsafe { &(*ptr).Job }
        };

        &payload.f0
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    /// Since `Job` only has one tag (namely, `Job`),
    /// convert it to `Job`'s payload.
    pub fn into_Job(mut self) -> R1 {
        let payload = {
            let ptr = (self.pointer as usize & !0b111) as *mut union_Job;
            let mut uninitialized = core::mem::MaybeUninit::uninit();
            let swapped = unsafe {
                core::mem::replace(
                    &mut (*ptr).Job,
                    core::mem::ManuallyDrop::new(uninitialized.assume_init()),
                )
            };

            core::mem::forget(self);

            core::mem::ManuallyDrop::into_inner(swapped)
        };

        payload.f0
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    /// Since `Job` only has one tag (namely, `Job`),
    /// convert it to `Job`'s payload.
    pub fn as_Job(&self) -> &R1 {
        let payload = {
            let ptr = (self.pointer as usize & !0b111) as *mut union_Job;

            unsafe { &(*ptr).Job }
        };

        &payload.f0
    }
}

impl Drop for Job {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn drop(&mut self) {
        // We only need to do any work if there's actually a heap-allocated payload.
        if let Some(storage) = self.storage() {
            let mut new_storage = storage.get();

            // Decrement the refcount
            let needs_dealloc = !new_storage.is_readonly() && new_storage.decrease();

            if needs_dealloc {
                // Drop the payload first.
                unsafe {
                    core::mem::ManuallyDrop::drop(&mut core::ptr::read(self.pointer).Job);
                }

                // Dealloc the pointer
                let alignment =
                    core::mem::align_of::<Self>().max(core::mem::align_of::<roc_std::Storage>());

                unsafe {
                    crate::roc_dealloc(storage.as_ptr().cast(), alignment as u32);
                }
            } else {
                // Write the storage back.
                storage.set(new_storage);
            }
        }
    }
}

impl Eq for Job {}

impl PartialEq for Job {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn eq(&self, other: &Self) -> bool {
        if self.discriminant() != other.discriminant() {
            return false;
        }

        unsafe { (*self.pointer).Job == (*other.pointer).Job }
    }
}

impl PartialOrd for Job {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        unsafe { (&*self.pointer).Job.partial_cmp(&(*other.pointer).Job) }
    }
}

impl Ord for Job {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        unsafe { (&*self.pointer).Job.cmp(&(*other.pointer).Job) }
    }
}

impl Clone for Job {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn clone(&self) -> Self {
        if let Some(storage) = self.storage() {
            let mut new_storage = storage.get();
            if !new_storage.is_readonly() {
                new_storage.increment_reference_count();
                storage.set(new_storage);
            }
        }

        Self {
            pointer: self.pointer,
        }
    }
}

impl core::hash::Hash for Job {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        unsafe { (*self.pointer).Job.hash(state) }
    }
}

impl core::fmt::Debug for Job {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Job::")?;

        unsafe { f.debug_tuple("Job").field(&(*self.pointer).Job).finish() }
    }
}

impl Tool {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// A tag named SystemTool, with the given payload.
    pub fn SystemTool(f0: SystemToolPayload) -> Self {
        Self { f0 }
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Since `SystemTool` only has one tag (namely, `SystemTool`),
    /// convert it to `SystemTool`'s payload.
    pub fn into_SystemTool(self) -> SystemToolPayload {
        self.f0
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Since `SystemTool` only has one tag (namely, `SystemTool`),
    /// convert it to `SystemTool`'s payload.
    pub fn as_SystemTool(&self) -> &SystemToolPayload {
        &self.f0
    }
}

impl core::fmt::Debug for Tool {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Tool::SystemTool").field(&self.f0).finish()
    }
}

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `StorageClass`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let storageclass = unimplemented!();
/// match storageclass {
///     StorageClass::Temporal => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `storageclass` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `StorageClass::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `StorageClass::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `StorageClass::NewFeature` is defined.
/// Specifically, when `storageclass` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `StorageClass::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum StorageClass {
    #[allow(missing_docs)] // documentation missing in model
    Temporal,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue),
}
impl std::convert::From<&str> for StorageClass {
    fn from(s: &str) -> Self {
        match s {
            "TEMPORAL" => StorageClass::Temporal,
            other => StorageClass::Unknown(crate::types::UnknownVariantValue(other.to_owned())),
        }
    }
}
impl std::str::FromStr for StorageClass {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(StorageClass::from(s))
    }
}
impl StorageClass {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            StorageClass::Temporal => "TEMPORAL",
            StorageClass::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &["TEMPORAL"]
    }
}
impl AsRef<str> for StorageClass {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// When writing a match expression against `UploadAvailability`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let uploadavailability = unimplemented!();
/// match uploadavailability {
///     UploadAvailability::Standard => { /* ... */ },
///     UploadAvailability::Streaming => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `uploadavailability` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `UploadAvailability::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `UploadAvailability::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `UploadAvailability::NewFeature` is defined.
/// Specifically, when `uploadavailability` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `UploadAvailability::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum UploadAvailability {
    #[allow(missing_docs)] // documentation missing in model
    Standard,
    #[allow(missing_docs)] // documentation missing in model
    Streaming,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue),
}
impl std::convert::From<&str> for UploadAvailability {
    fn from(s: &str) -> Self {
        match s {
            "STANDARD" => UploadAvailability::Standard,
            "STREAMING" => UploadAvailability::Streaming,
            other => {
                UploadAvailability::Unknown(crate::types::UnknownVariantValue(other.to_owned()))
            }
        }
    }
}
impl std::str::FromStr for UploadAvailability {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(UploadAvailability::from(s))
    }
}
impl UploadAvailability {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            UploadAvailability::Standard => "STANDARD",
            UploadAvailability::Streaming => "STREAMING",
            UploadAvailability::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &["STANDARD", "STREAMING"]
    }
}
impl AsRef<str> for UploadAvailability {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>A metadata entry for a folder or object.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct Item {
    /// <p>The name of the item.</p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
    /// <p>The item type (folder or object).</p>
    #[doc(hidden)]
    pub r#type: std::option::Option<crate::model::ItemType>,
    /// <p>The ETag that represents a unique instance of the item.</p>
    #[doc(hidden)]
    pub e_tag: std::option::Option<std::string::String>,
    /// <p>The date and time that the item was last modified.</p>
    #[doc(hidden)]
    pub last_modified: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The content type of the item.</p>
    #[doc(hidden)]
    pub content_type: std::option::Option<std::string::String>,
    /// <p>The length of the item in bytes.</p>
    #[doc(hidden)]
    pub content_length: std::option::Option<i64>,
}
impl Item {
    /// <p>The name of the item.</p>
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The item type (folder or object).</p>
    pub fn r#type(&self) -> std::option::Option<&crate::model::ItemType> {
        self.r#type.as_ref()
    }
    /// <p>The ETag that represents a unique instance of the item.</p>
    pub fn e_tag(&self) -> std::option::Option<&str> {
        self.e_tag.as_deref()
    }
    /// <p>The date and time that the item was last modified.</p>
    pub fn last_modified(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.last_modified.as_ref()
    }
    /// <p>The content type of the item.</p>
    pub fn content_type(&self) -> std::option::Option<&str> {
        self.content_type.as_deref()
    }
    /// <p>The length of the item in bytes.</p>
    pub fn content_length(&self) -> std::option::Option<i64> {
        self.content_length
    }
}
/// See [`Item`](crate::model::Item).
pub mod item {

    /// A builder for [`Item`](crate::model::Item).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) name: std::option::Option<std::string::String>,
        pub(crate) r#type: std::option::Option<crate::model::ItemType>,
        pub(crate) e_tag: std::option::Option<std::string::String>,
        pub(crate) last_modified: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) content_type: std::option::Option<std::string::String>,
        pub(crate) content_length: std::option::Option<i64>,
    }
    impl Builder {
        /// <p>The name of the item.</p>
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        /// <p>The name of the item.</p>
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input;
            self
        }
        /// <p>The item type (folder or object).</p>
        pub fn r#type(mut self, input: crate::model::ItemType) -> Self {
            self.r#type = Some(input);
            self
        }
        /// <p>The item type (folder or object).</p>
        pub fn set_type(mut self, input: std::option::Option<crate::model::ItemType>) -> Self {
            self.r#type = input;
            self
        }
        /// <p>The ETag that represents a unique instance of the item.</p>
        pub fn e_tag(mut self, input: impl Into<std::string::String>) -> Self {
            self.e_tag = Some(input.into());
            self
        }
        /// <p>The ETag that represents a unique instance of the item.</p>
        pub fn set_e_tag(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.e_tag = input;
            self
        }
        /// <p>The date and time that the item was last modified.</p>
        pub fn last_modified(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.last_modified = Some(input);
            self
        }
        /// <p>The date and time that the item was last modified.</p>
        pub fn set_last_modified(
            mut self,
            input: std::option::Option<aws_smithy_types::DateTime>,
        ) -> Self {
            self.last_modified = input;
            self
        }
        /// <p>The content type of the item.</p>
        pub fn content_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.content_type = Some(input.into());
            self
        }
        /// <p>The content type of the item.</p>
        pub fn set_content_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.content_type = input;
            self
        }
        /// <p>The length of the item in bytes.</p>
        pub fn content_length(mut self, input: i64) -> Self {
            self.content_length = Some(input);
            self
        }
        /// <p>The length of the item in bytes.</p>
        pub fn set_content_length(mut self, input: std::option::Option<i64>) -> Self {
            self.content_length = input;
            self
        }
        /// Consumes the builder and constructs a [`Item`](crate::model::Item).
        pub fn build(self) -> crate::model::Item {
            crate::model::Item {
                name: self.name,
                r#type: self.r#type,
                e_tag: self.e_tag,
                last_modified: self.last_modified,
                content_type: self.content_type,
                content_length: self.content_length,
            }
        }
    }
}
impl Item {
    /// Creates a new builder-style object to manufacture [`Item`](crate::model::Item).
    pub fn builder() -> crate::model::item::Builder {
        crate::model::item::Builder::default()
    }
}

/// When writing a match expression against `ItemType`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let itemtype = unimplemented!();
/// match itemtype {
///     ItemType::Folder => { /* ... */ },
///     ItemType::Object => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `itemtype` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `ItemType::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `ItemType::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `ItemType::NewFeature` is defined.
/// Specifically, when `itemtype` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `ItemType::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum ItemType {
    #[allow(missing_docs)] // documentation missing in model
    Folder,
    #[allow(missing_docs)] // documentation missing in model
    Object,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue),
}
impl std::convert::From<&str> for ItemType {
    fn from(s: &str) -> Self {
        match s {
            "FOLDER" => ItemType::Folder,
            "OBJECT" => ItemType::Object,
            other => ItemType::Unknown(crate::types::UnknownVariantValue(other.to_owned())),
        }
    }
}
impl std::str::FromStr for ItemType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ItemType::from(s))
    }
}
impl ItemType {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ItemType::Folder => "FOLDER",
            ItemType::Object => "OBJECT",
            ItemType::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &["FOLDER", "OBJECT"]
    }
}
impl AsRef<str> for ItemType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

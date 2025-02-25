// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetAppMonitorDataOutput {
    /// <p>The events that RUM collected that match your request.</p>
    #[doc(hidden)]
    pub events: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>A token that you can use in a subsequent operation to retrieve the next set of results.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl GetAppMonitorDataOutput {
    /// <p>The events that RUM collected that match your request.</p>
    pub fn events(&self) -> std::option::Option<&[std::string::String]> {
        self.events.as_deref()
    }
    /// <p>A token that you can use in a subsequent operation to retrieve the next set of results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
/// See [`GetAppMonitorDataOutput`](crate::output::GetAppMonitorDataOutput).
pub mod get_app_monitor_data_output {

    /// A builder for [`GetAppMonitorDataOutput`](crate::output::GetAppMonitorDataOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) events: std::option::Option<std::vec::Vec<std::string::String>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `events`.
        ///
        /// To override the contents of this collection use [`set_events`](Self::set_events).
        ///
        /// <p>The events that RUM collected that match your request.</p>
        pub fn events(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.events.unwrap_or_default();
            v.push(input.into());
            self.events = Some(v);
            self
        }
        /// <p>The events that RUM collected that match your request.</p>
        pub fn set_events(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.events = input;
            self
        }
        /// <p>A token that you can use in a subsequent operation to retrieve the next set of results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>A token that you can use in a subsequent operation to retrieve the next set of results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`GetAppMonitorDataOutput`](crate::output::GetAppMonitorDataOutput).
        pub fn build(self) -> crate::output::GetAppMonitorDataOutput {
            crate::output::GetAppMonitorDataOutput {
                events: self.events,
                next_token: self.next_token,
            }
        }
    }
}
impl GetAppMonitorDataOutput {
    /// Creates a new builder-style object to manufacture [`GetAppMonitorDataOutput`](crate::output::GetAppMonitorDataOutput).
    pub fn builder() -> crate::output::get_app_monitor_data_output::Builder {
        crate::output::get_app_monitor_data_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CreateAppMonitorOutput {
    /// <p>The unique ID of the new app monitor.</p>
    #[doc(hidden)]
    pub id: std::option::Option<std::string::String>,
}
impl CreateAppMonitorOutput {
    /// <p>The unique ID of the new app monitor.</p>
    pub fn id(&self) -> std::option::Option<&str> {
        self.id.as_deref()
    }
}
/// See [`CreateAppMonitorOutput`](crate::output::CreateAppMonitorOutput).
pub mod create_app_monitor_output {

    /// A builder for [`CreateAppMonitorOutput`](crate::output::CreateAppMonitorOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The unique ID of the new app monitor.</p>
        pub fn id(mut self, input: impl Into<std::string::String>) -> Self {
            self.id = Some(input.into());
            self
        }
        /// <p>The unique ID of the new app monitor.</p>
        pub fn set_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.id = input;
            self
        }
        /// Consumes the builder and constructs a [`CreateAppMonitorOutput`](crate::output::CreateAppMonitorOutput).
        pub fn build(self) -> crate::output::CreateAppMonitorOutput {
            crate::output::CreateAppMonitorOutput { id: self.id }
        }
    }
}
impl CreateAppMonitorOutput {
    /// Creates a new builder-style object to manufacture [`CreateAppMonitorOutput`](crate::output::CreateAppMonitorOutput).
    pub fn builder() -> crate::output::create_app_monitor_output::Builder {
        crate::output::create_app_monitor_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListAppMonitorsOutput {
    /// <p>A token that you can use in a subsequent operation to retrieve the next set of results.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>An array of structures that contain information about the returned app monitors.</p>
    #[doc(hidden)]
    pub app_monitor_summaries: std::option::Option<std::vec::Vec<crate::model::AppMonitorSummary>>,
}
impl ListAppMonitorsOutput {
    /// <p>A token that you can use in a subsequent operation to retrieve the next set of results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>An array of structures that contain information about the returned app monitors.</p>
    pub fn app_monitor_summaries(&self) -> std::option::Option<&[crate::model::AppMonitorSummary]> {
        self.app_monitor_summaries.as_deref()
    }
}
/// See [`ListAppMonitorsOutput`](crate::output::ListAppMonitorsOutput).
pub mod list_app_monitors_output {

    /// A builder for [`ListAppMonitorsOutput`](crate::output::ListAppMonitorsOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) next_token: std::option::Option<std::string::String>,
        pub(crate) app_monitor_summaries:
            std::option::Option<std::vec::Vec<crate::model::AppMonitorSummary>>,
    }
    impl Builder {
        /// <p>A token that you can use in a subsequent operation to retrieve the next set of results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>A token that you can use in a subsequent operation to retrieve the next set of results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Appends an item to `app_monitor_summaries`.
        ///
        /// To override the contents of this collection use [`set_app_monitor_summaries`](Self::set_app_monitor_summaries).
        ///
        /// <p>An array of structures that contain information about the returned app monitors.</p>
        pub fn app_monitor_summaries(mut self, input: crate::model::AppMonitorSummary) -> Self {
            let mut v = self.app_monitor_summaries.unwrap_or_default();
            v.push(input);
            self.app_monitor_summaries = Some(v);
            self
        }
        /// <p>An array of structures that contain information about the returned app monitors.</p>
        pub fn set_app_monitor_summaries(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::AppMonitorSummary>>,
        ) -> Self {
            self.app_monitor_summaries = input;
            self
        }
        /// Consumes the builder and constructs a [`ListAppMonitorsOutput`](crate::output::ListAppMonitorsOutput).
        pub fn build(self) -> crate::output::ListAppMonitorsOutput {
            crate::output::ListAppMonitorsOutput {
                next_token: self.next_token,
                app_monitor_summaries: self.app_monitor_summaries,
            }
        }
    }
}
impl ListAppMonitorsOutput {
    /// Creates a new builder-style object to manufacture [`ListAppMonitorsOutput`](crate::output::ListAppMonitorsOutput).
    pub fn builder() -> crate::output::list_app_monitors_output::Builder {
        crate::output::list_app_monitors_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeleteAppMonitorOutput {}
/// See [`DeleteAppMonitorOutput`](crate::output::DeleteAppMonitorOutput).
pub mod delete_app_monitor_output {

    /// A builder for [`DeleteAppMonitorOutput`](crate::output::DeleteAppMonitorOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteAppMonitorOutput`](crate::output::DeleteAppMonitorOutput).
        pub fn build(self) -> crate::output::DeleteAppMonitorOutput {
            crate::output::DeleteAppMonitorOutput {}
        }
    }
}
impl DeleteAppMonitorOutput {
    /// Creates a new builder-style object to manufacture [`DeleteAppMonitorOutput`](crate::output::DeleteAppMonitorOutput).
    pub fn builder() -> crate::output::delete_app_monitor_output::Builder {
        crate::output::delete_app_monitor_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UpdateAppMonitorOutput {}
/// See [`UpdateAppMonitorOutput`](crate::output::UpdateAppMonitorOutput).
pub mod update_app_monitor_output {

    /// A builder for [`UpdateAppMonitorOutput`](crate::output::UpdateAppMonitorOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`UpdateAppMonitorOutput`](crate::output::UpdateAppMonitorOutput).
        pub fn build(self) -> crate::output::UpdateAppMonitorOutput {
            crate::output::UpdateAppMonitorOutput {}
        }
    }
}
impl UpdateAppMonitorOutput {
    /// Creates a new builder-style object to manufacture [`UpdateAppMonitorOutput`](crate::output::UpdateAppMonitorOutput).
    pub fn builder() -> crate::output::update_app_monitor_output::Builder {
        crate::output::update_app_monitor_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetAppMonitorOutput {
    /// <p>A structure containing all the configuration information for the app monitor.</p>
    #[doc(hidden)]
    pub app_monitor: std::option::Option<crate::model::AppMonitor>,
}
impl GetAppMonitorOutput {
    /// <p>A structure containing all the configuration information for the app monitor.</p>
    pub fn app_monitor(&self) -> std::option::Option<&crate::model::AppMonitor> {
        self.app_monitor.as_ref()
    }
}
/// See [`GetAppMonitorOutput`](crate::output::GetAppMonitorOutput).
pub mod get_app_monitor_output {

    /// A builder for [`GetAppMonitorOutput`](crate::output::GetAppMonitorOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) app_monitor: std::option::Option<crate::model::AppMonitor>,
    }
    impl Builder {
        /// <p>A structure containing all the configuration information for the app monitor.</p>
        pub fn app_monitor(mut self, input: crate::model::AppMonitor) -> Self {
            self.app_monitor = Some(input);
            self
        }
        /// <p>A structure containing all the configuration information for the app monitor.</p>
        pub fn set_app_monitor(
            mut self,
            input: std::option::Option<crate::model::AppMonitor>,
        ) -> Self {
            self.app_monitor = input;
            self
        }
        /// Consumes the builder and constructs a [`GetAppMonitorOutput`](crate::output::GetAppMonitorOutput).
        pub fn build(self) -> crate::output::GetAppMonitorOutput {
            crate::output::GetAppMonitorOutput {
                app_monitor: self.app_monitor,
            }
        }
    }
}
impl GetAppMonitorOutput {
    /// Creates a new builder-style object to manufacture [`GetAppMonitorOutput`](crate::output::GetAppMonitorOutput).
    pub fn builder() -> crate::output::get_app_monitor_output::Builder {
        crate::output::get_app_monitor_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UntagResourceOutput {}
/// See [`UntagResourceOutput`](crate::output::UntagResourceOutput).
pub mod untag_resource_output {

    /// A builder for [`UntagResourceOutput`](crate::output::UntagResourceOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`UntagResourceOutput`](crate::output::UntagResourceOutput).
        pub fn build(self) -> crate::output::UntagResourceOutput {
            crate::output::UntagResourceOutput {}
        }
    }
}
impl UntagResourceOutput {
    /// Creates a new builder-style object to manufacture [`UntagResourceOutput`](crate::output::UntagResourceOutput).
    pub fn builder() -> crate::output::untag_resource_output::Builder {
        crate::output::untag_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct TagResourceOutput {}
/// See [`TagResourceOutput`](crate::output::TagResourceOutput).
pub mod tag_resource_output {

    /// A builder for [`TagResourceOutput`](crate::output::TagResourceOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`TagResourceOutput`](crate::output::TagResourceOutput).
        pub fn build(self) -> crate::output::TagResourceOutput {
            crate::output::TagResourceOutput {}
        }
    }
}
impl TagResourceOutput {
    /// Creates a new builder-style object to manufacture [`TagResourceOutput`](crate::output::TagResourceOutput).
    pub fn builder() -> crate::output::tag_resource_output::Builder {
        crate::output::tag_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct PutRumEventsOutput {}
/// See [`PutRumEventsOutput`](crate::output::PutRumEventsOutput).
pub mod put_rum_events_output {

    /// A builder for [`PutRumEventsOutput`](crate::output::PutRumEventsOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`PutRumEventsOutput`](crate::output::PutRumEventsOutput).
        pub fn build(self) -> crate::output::PutRumEventsOutput {
            crate::output::PutRumEventsOutput {}
        }
    }
}
impl PutRumEventsOutput {
    /// Creates a new builder-style object to manufacture [`PutRumEventsOutput`](crate::output::PutRumEventsOutput).
    pub fn builder() -> crate::output::put_rum_events_output::Builder {
        crate::output::put_rum_events_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListTagsForResourceOutput {
    /// <p>The ARN of the resource that you are viewing.</p>
    #[doc(hidden)]
    pub resource_arn: std::option::Option<std::string::String>,
    /// <p>The list of tag keys and values associated with the resource you specified.</p>
    #[doc(hidden)]
    pub tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl ListTagsForResourceOutput {
    /// <p>The ARN of the resource that you are viewing.</p>
    pub fn resource_arn(&self) -> std::option::Option<&str> {
        self.resource_arn.as_deref()
    }
    /// <p>The list of tag keys and values associated with the resource you specified.</p>
    pub fn tags(
        &self,
    ) -> std::option::Option<&std::collections::HashMap<std::string::String, std::string::String>>
    {
        self.tags.as_ref()
    }
}
/// See [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
pub mod list_tags_for_resource_output {

    /// A builder for [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) resource_arn: std::option::Option<std::string::String>,
        pub(crate) tags: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    }
    impl Builder {
        /// <p>The ARN of the resource that you are viewing.</p>
        pub fn resource_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.resource_arn = Some(input.into());
            self
        }
        /// <p>The ARN of the resource that you are viewing.</p>
        pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.resource_arn = input;
            self
        }
        /// Adds a key-value pair to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>The list of tag keys and values associated with the resource you specified.</p>
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.tags.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.tags = Some(hash_map);
            self
        }
        /// <p>The list of tag keys and values associated with the resource you specified.</p>
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.tags = input;
            self
        }
        /// Consumes the builder and constructs a [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
        pub fn build(self) -> crate::output::ListTagsForResourceOutput {
            crate::output::ListTagsForResourceOutput {
                resource_arn: self.resource_arn,
                tags: self.tags,
            }
        }
    }
}
impl ListTagsForResourceOutput {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
    pub fn builder() -> crate::output::list_tags_for_resource_output::Builder {
        crate::output::list_tags_for_resource_output::Builder::default()
    }
}

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Paginator for [`GetResourcePolicies`](crate::operation::GetResourcePolicies)
pub struct GetResourcePoliciesPaginator {
    handle: std::sync::Arc<crate::client::Handle>,
    builder: crate::input::get_resource_policies_input::Builder,
    stop_on_duplicate_token: bool,
}

impl GetResourcePoliciesPaginator {
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle>,
        builder: crate::input::get_resource_policies_input::Builder,
    ) -> Self {
        Self {
            handle,
            builder,
            stop_on_duplicate_token: true,
        }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `max_results`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.max_results = Some(limit);
        self
    }

    /// Create a flattened paginator
    ///
    /// This paginator automatically flattens results using `resource_policies`. Queries to the underlying service
    /// are dispatched lazily.
    pub fn items(self) -> crate::paginator::GetResourcePoliciesPaginatorItems {
        crate::paginator::GetResourcePoliciesPaginatorItems(self)
    }

    /// Stop paginating when the service returns the same pagination token twice in a row.
    ///
    /// Defaults to true.
    ///
    /// For certain operations, it may be useful to continue on duplicate token. For example,
    /// if an operation is for tailing a log file in real-time, then continuing may be desired.
    /// This option can be set to `false` to accommodate these use cases.
    pub fn stop_on_duplicate_token(mut self, stop_on_duplicate_token: bool) -> Self {
        self.stop_on_duplicate_token = stop_on_duplicate_token;
        self
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::output::GetResourcePoliciesOutput,
            aws_smithy_http::result::SdkError<crate::error::GetResourcePoliciesError>,
        >,
    > + Unpin {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder
                    .build()
                    .map_err(aws_smithy_http::result::SdkError::construction_failure)
                {
                    Ok(input) => input,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input
                        .make_operation(&handle.conf)
                        .await
                        .map_err(aws_smithy_http::result::SdkError::construction_failure)
                    {
                        Ok(op) => op,
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        Ok(ref resp) => {
                            let new_token = crate::lens::reflens_structure_crate_output_get_resource_policies_output_next_token(resp);
                            let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                            if !is_empty
                                && new_token == input.next_token.as_ref()
                                && self.stop_on_duplicate_token
                            {
                                true
                            } else {
                                input.next_token = new_token.cloned();
                                is_empty
                            }
                        }
                        Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}

/// Paginator for [`ListIncidentRecords`](crate::operation::ListIncidentRecords)
pub struct ListIncidentRecordsPaginator {
    handle: std::sync::Arc<crate::client::Handle>,
    builder: crate::input::list_incident_records_input::Builder,
    stop_on_duplicate_token: bool,
}

impl ListIncidentRecordsPaginator {
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle>,
        builder: crate::input::list_incident_records_input::Builder,
    ) -> Self {
        Self {
            handle,
            builder,
            stop_on_duplicate_token: true,
        }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `max_results`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.max_results = Some(limit);
        self
    }

    /// Create a flattened paginator
    ///
    /// This paginator automatically flattens results using `incident_record_summaries`. Queries to the underlying service
    /// are dispatched lazily.
    pub fn items(self) -> crate::paginator::ListIncidentRecordsPaginatorItems {
        crate::paginator::ListIncidentRecordsPaginatorItems(self)
    }

    /// Stop paginating when the service returns the same pagination token twice in a row.
    ///
    /// Defaults to true.
    ///
    /// For certain operations, it may be useful to continue on duplicate token. For example,
    /// if an operation is for tailing a log file in real-time, then continuing may be desired.
    /// This option can be set to `false` to accommodate these use cases.
    pub fn stop_on_duplicate_token(mut self, stop_on_duplicate_token: bool) -> Self {
        self.stop_on_duplicate_token = stop_on_duplicate_token;
        self
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::output::ListIncidentRecordsOutput,
            aws_smithy_http::result::SdkError<crate::error::ListIncidentRecordsError>,
        >,
    > + Unpin {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder
                    .build()
                    .map_err(aws_smithy_http::result::SdkError::construction_failure)
                {
                    Ok(input) => input,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input
                        .make_operation(&handle.conf)
                        .await
                        .map_err(aws_smithy_http::result::SdkError::construction_failure)
                    {
                        Ok(op) => op,
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        Ok(ref resp) => {
                            let new_token = crate::lens::reflens_structure_crate_output_list_incident_records_output_next_token(resp);
                            let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                            if !is_empty
                                && new_token == input.next_token.as_ref()
                                && self.stop_on_duplicate_token
                            {
                                true
                            } else {
                                input.next_token = new_token.cloned();
                                is_empty
                            }
                        }
                        Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}

/// Paginator for [`ListRelatedItems`](crate::operation::ListRelatedItems)
pub struct ListRelatedItemsPaginator {
    handle: std::sync::Arc<crate::client::Handle>,
    builder: crate::input::list_related_items_input::Builder,
    stop_on_duplicate_token: bool,
}

impl ListRelatedItemsPaginator {
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle>,
        builder: crate::input::list_related_items_input::Builder,
    ) -> Self {
        Self {
            handle,
            builder,
            stop_on_duplicate_token: true,
        }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `max_results`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.max_results = Some(limit);
        self
    }

    /// Create a flattened paginator
    ///
    /// This paginator automatically flattens results using `related_items`. Queries to the underlying service
    /// are dispatched lazily.
    pub fn items(self) -> crate::paginator::ListRelatedItemsPaginatorItems {
        crate::paginator::ListRelatedItemsPaginatorItems(self)
    }

    /// Stop paginating when the service returns the same pagination token twice in a row.
    ///
    /// Defaults to true.
    ///
    /// For certain operations, it may be useful to continue on duplicate token. For example,
    /// if an operation is for tailing a log file in real-time, then continuing may be desired.
    /// This option can be set to `false` to accommodate these use cases.
    pub fn stop_on_duplicate_token(mut self, stop_on_duplicate_token: bool) -> Self {
        self.stop_on_duplicate_token = stop_on_duplicate_token;
        self
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::output::ListRelatedItemsOutput,
            aws_smithy_http::result::SdkError<crate::error::ListRelatedItemsError>,
        >,
    > + Unpin {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder
                    .build()
                    .map_err(aws_smithy_http::result::SdkError::construction_failure)
                {
                    Ok(input) => input,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input
                        .make_operation(&handle.conf)
                        .await
                        .map_err(aws_smithy_http::result::SdkError::construction_failure)
                    {
                        Ok(op) => op,
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        Ok(ref resp) => {
                            let new_token = crate::lens::reflens_structure_crate_output_list_related_items_output_next_token(resp);
                            let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                            if !is_empty
                                && new_token == input.next_token.as_ref()
                                && self.stop_on_duplicate_token
                            {
                                true
                            } else {
                                input.next_token = new_token.cloned();
                                is_empty
                            }
                        }
                        Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}

/// Paginator for [`ListReplicationSets`](crate::operation::ListReplicationSets)
pub struct ListReplicationSetsPaginator {
    handle: std::sync::Arc<crate::client::Handle>,
    builder: crate::input::list_replication_sets_input::Builder,
    stop_on_duplicate_token: bool,
}

impl ListReplicationSetsPaginator {
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle>,
        builder: crate::input::list_replication_sets_input::Builder,
    ) -> Self {
        Self {
            handle,
            builder,
            stop_on_duplicate_token: true,
        }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `max_results`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.max_results = Some(limit);
        self
    }

    /// Create a flattened paginator
    ///
    /// This paginator automatically flattens results using `replication_set_arns`. Queries to the underlying service
    /// are dispatched lazily.
    pub fn items(self) -> crate::paginator::ListReplicationSetsPaginatorItems {
        crate::paginator::ListReplicationSetsPaginatorItems(self)
    }

    /// Stop paginating when the service returns the same pagination token twice in a row.
    ///
    /// Defaults to true.
    ///
    /// For certain operations, it may be useful to continue on duplicate token. For example,
    /// if an operation is for tailing a log file in real-time, then continuing may be desired.
    /// This option can be set to `false` to accommodate these use cases.
    pub fn stop_on_duplicate_token(mut self, stop_on_duplicate_token: bool) -> Self {
        self.stop_on_duplicate_token = stop_on_duplicate_token;
        self
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::output::ListReplicationSetsOutput,
            aws_smithy_http::result::SdkError<crate::error::ListReplicationSetsError>,
        >,
    > + Unpin {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder
                    .build()
                    .map_err(aws_smithy_http::result::SdkError::construction_failure)
                {
                    Ok(input) => input,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input
                        .make_operation(&handle.conf)
                        .await
                        .map_err(aws_smithy_http::result::SdkError::construction_failure)
                    {
                        Ok(op) => op,
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        Ok(ref resp) => {
                            let new_token = crate::lens::reflens_structure_crate_output_list_replication_sets_output_next_token(resp);
                            let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                            if !is_empty
                                && new_token == input.next_token.as_ref()
                                && self.stop_on_duplicate_token
                            {
                                true
                            } else {
                                input.next_token = new_token.cloned();
                                is_empty
                            }
                        }
                        Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}

/// Paginator for [`ListResponsePlans`](crate::operation::ListResponsePlans)
pub struct ListResponsePlansPaginator {
    handle: std::sync::Arc<crate::client::Handle>,
    builder: crate::input::list_response_plans_input::Builder,
    stop_on_duplicate_token: bool,
}

impl ListResponsePlansPaginator {
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle>,
        builder: crate::input::list_response_plans_input::Builder,
    ) -> Self {
        Self {
            handle,
            builder,
            stop_on_duplicate_token: true,
        }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `max_results`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.max_results = Some(limit);
        self
    }

    /// Create a flattened paginator
    ///
    /// This paginator automatically flattens results using `response_plan_summaries`. Queries to the underlying service
    /// are dispatched lazily.
    pub fn items(self) -> crate::paginator::ListResponsePlansPaginatorItems {
        crate::paginator::ListResponsePlansPaginatorItems(self)
    }

    /// Stop paginating when the service returns the same pagination token twice in a row.
    ///
    /// Defaults to true.
    ///
    /// For certain operations, it may be useful to continue on duplicate token. For example,
    /// if an operation is for tailing a log file in real-time, then continuing may be desired.
    /// This option can be set to `false` to accommodate these use cases.
    pub fn stop_on_duplicate_token(mut self, stop_on_duplicate_token: bool) -> Self {
        self.stop_on_duplicate_token = stop_on_duplicate_token;
        self
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::output::ListResponsePlansOutput,
            aws_smithy_http::result::SdkError<crate::error::ListResponsePlansError>,
        >,
    > + Unpin {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder
                    .build()
                    .map_err(aws_smithy_http::result::SdkError::construction_failure)
                {
                    Ok(input) => input,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input
                        .make_operation(&handle.conf)
                        .await
                        .map_err(aws_smithy_http::result::SdkError::construction_failure)
                    {
                        Ok(op) => op,
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        Ok(ref resp) => {
                            let new_token = crate::lens::reflens_structure_crate_output_list_response_plans_output_next_token(resp);
                            let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                            if !is_empty
                                && new_token == input.next_token.as_ref()
                                && self.stop_on_duplicate_token
                            {
                                true
                            } else {
                                input.next_token = new_token.cloned();
                                is_empty
                            }
                        }
                        Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}

/// Paginator for [`ListTimelineEvents`](crate::operation::ListTimelineEvents)
pub struct ListTimelineEventsPaginator {
    handle: std::sync::Arc<crate::client::Handle>,
    builder: crate::input::list_timeline_events_input::Builder,
    stop_on_duplicate_token: bool,
}

impl ListTimelineEventsPaginator {
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle>,
        builder: crate::input::list_timeline_events_input::Builder,
    ) -> Self {
        Self {
            handle,
            builder,
            stop_on_duplicate_token: true,
        }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `max_results`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.max_results = Some(limit);
        self
    }

    /// Create a flattened paginator
    ///
    /// This paginator automatically flattens results using `event_summaries`. Queries to the underlying service
    /// are dispatched lazily.
    pub fn items(self) -> crate::paginator::ListTimelineEventsPaginatorItems {
        crate::paginator::ListTimelineEventsPaginatorItems(self)
    }

    /// Stop paginating when the service returns the same pagination token twice in a row.
    ///
    /// Defaults to true.
    ///
    /// For certain operations, it may be useful to continue on duplicate token. For example,
    /// if an operation is for tailing a log file in real-time, then continuing may be desired.
    /// This option can be set to `false` to accommodate these use cases.
    pub fn stop_on_duplicate_token(mut self, stop_on_duplicate_token: bool) -> Self {
        self.stop_on_duplicate_token = stop_on_duplicate_token;
        self
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::output::ListTimelineEventsOutput,
            aws_smithy_http::result::SdkError<crate::error::ListTimelineEventsError>,
        >,
    > + Unpin {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder
                    .build()
                    .map_err(aws_smithy_http::result::SdkError::construction_failure)
                {
                    Ok(input) => input,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input
                        .make_operation(&handle.conf)
                        .await
                        .map_err(aws_smithy_http::result::SdkError::construction_failure)
                    {
                        Ok(op) => op,
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        Ok(ref resp) => {
                            let new_token = crate::lens::reflens_structure_crate_output_list_timeline_events_output_next_token(resp);
                            let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                            if !is_empty
                                && new_token == input.next_token.as_ref()
                                && self.stop_on_duplicate_token
                            {
                                true
                            } else {
                                input.next_token = new_token.cloned();
                                is_empty
                            }
                        }
                        Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}

/// Flattened paginator for `GetResourcePoliciesPaginator`
///
/// This is created with [`.items()`](GetResourcePoliciesPaginator::items)
pub struct GetResourcePoliciesPaginatorItems(GetResourcePoliciesPaginator);

impl GetResourcePoliciesPaginatorItems {
    /// Create the pagination stream
    ///
    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
    ///
    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::model::ResourcePolicy,
            aws_smithy_http::result::SdkError<crate::error::GetResourcePoliciesError>,
        >,
    > + Unpin {
        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| {
            crate::lens::lens_structure_crate_output_get_resource_policies_output_resource_policies(
                page,
            )
            .unwrap_or_default()
            .into_iter()
        })
    }
}

/// Flattened paginator for `ListIncidentRecordsPaginator`
///
/// This is created with [`.items()`](ListIncidentRecordsPaginator::items)
pub struct ListIncidentRecordsPaginatorItems(ListIncidentRecordsPaginator);

impl ListIncidentRecordsPaginatorItems {
    /// Create the pagination stream
    ///
    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
    ///
    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::model::IncidentRecordSummary,
            aws_smithy_http::result::SdkError<crate::error::ListIncidentRecordsError>,
        >,
    > + Unpin {
        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| crate::lens::lens_structure_crate_output_list_incident_records_output_incident_record_summaries(page).unwrap_or_default().into_iter())
    }
}

/// Flattened paginator for `ListRelatedItemsPaginator`
///
/// This is created with [`.items()`](ListRelatedItemsPaginator::items)
pub struct ListRelatedItemsPaginatorItems(ListRelatedItemsPaginator);

impl ListRelatedItemsPaginatorItems {
    /// Create the pagination stream
    ///
    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
    ///
    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::model::RelatedItem,
            aws_smithy_http::result::SdkError<crate::error::ListRelatedItemsError>,
        >,
    > + Unpin {
        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| {
            crate::lens::lens_structure_crate_output_list_related_items_output_related_items(page)
                .unwrap_or_default()
                .into_iter()
        })
    }
}

/// Flattened paginator for `ListReplicationSetsPaginator`
///
/// This is created with [`.items()`](ListReplicationSetsPaginator::items)
pub struct ListReplicationSetsPaginatorItems(ListReplicationSetsPaginator);

impl ListReplicationSetsPaginatorItems {
    /// Create the pagination stream
    ///
    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
    ///
    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            std::string::String,
            aws_smithy_http::result::SdkError<crate::error::ListReplicationSetsError>,
        >,
    > + Unpin {
        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| crate::lens::lens_structure_crate_output_list_replication_sets_output_replication_set_arns(page).unwrap_or_default().into_iter())
    }
}

/// Flattened paginator for `ListResponsePlansPaginator`
///
/// This is created with [`.items()`](ListResponsePlansPaginator::items)
pub struct ListResponsePlansPaginatorItems(ListResponsePlansPaginator);

impl ListResponsePlansPaginatorItems {
    /// Create the pagination stream
    ///
    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
    ///
    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::model::ResponsePlanSummary,
            aws_smithy_http::result::SdkError<crate::error::ListResponsePlansError>,
        >,
    > + Unpin {
        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| crate::lens::lens_structure_crate_output_list_response_plans_output_response_plan_summaries(page).unwrap_or_default().into_iter())
    }
}

/// Flattened paginator for `ListTimelineEventsPaginator`
///
/// This is created with [`.items()`](ListTimelineEventsPaginator::items)
pub struct ListTimelineEventsPaginatorItems(ListTimelineEventsPaginator);

impl ListTimelineEventsPaginatorItems {
    /// Create the pagination stream
    ///
    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
    ///
    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::model::EventSummary,
            aws_smithy_http::result::SdkError<crate::error::ListTimelineEventsError>,
        >,
    > + Unpin {
        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| {
            crate::lens::lens_structure_crate_output_list_timeline_events_output_event_summaries(
                page,
            )
            .unwrap_or_default()
            .into_iter()
        })
    }
}

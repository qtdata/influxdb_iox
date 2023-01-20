use std::sync::Arc;

use self::{
    commit::Commit, files_filter::FilesFilter, partition_error_sink::PartitionErrorSink,
    partition_files_source::PartitionFilesSource, partition_filter::PartitionFilter,
    partitions_source::PartitionsSource,
};

pub mod commit;
pub mod file_filter;
pub mod files_filter;
pub mod hardcoded;
pub mod partition_error_sink;
pub mod partition_files_source;
pub mod partition_filter;
pub mod partitions_source;

#[derive(Debug)]
pub struct Components {
    pub partitions_source: Arc<dyn PartitionsSource>,
    pub partition_files_source: Arc<dyn PartitionFilesSource>,
    pub files_filter: Arc<dyn FilesFilter>,
    pub partition_filter: Arc<dyn PartitionFilter>,
    pub partition_error_sink: Arc<dyn PartitionErrorSink>,
    pub commit: Arc<dyn Commit>,
}
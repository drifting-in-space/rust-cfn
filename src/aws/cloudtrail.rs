/// The [`AWS::CloudTrail::Trail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html) resource.
pub struct Trail {
    properties: TrailProperties
}

/// Properties for the `Trail` resource.
#[derive(Serialize, Deserialize)]
pub struct TrailProperties {
    #[serde(rename="CloudWatchLogsLogGroupArn")]
    pub cloud_watch_logs_log_group_arn: String,
    #[serde(rename="CloudWatchLogsRoleArn")]
    pub cloud_watch_logs_role_arn: String,
    #[serde(rename="EnableLogFileValidation")]
    pub enable_log_file_validation: bool,
    #[serde(rename="EventSelectors")]
    pub event_selectors: Vec<self::trail::EventSelector>,
    #[serde(rename="IncludeGlobalServiceEvents")]
    pub include_global_service_events: bool,
    #[serde(rename="IsLogging")]
    pub is_logging: bool,
    #[serde(rename="IsMultiRegionTrail")]
    pub is_multi_region_trail: bool,
    #[serde(rename="KMSKeyId")]
    pub kms_key_id: String,
    #[serde(rename="S3BucketName")]
    pub s3_bucket_name: String,
    #[serde(rename="S3KeyPrefix")]
    pub s3_key_prefix: String,
    #[serde(rename="SnsTopicName")]
    pub sns_topic_name: String,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    #[serde(rename="TrailName")]
    pub trail_name: String,
}

impl<'a> ::Resource<'a> for Trail {
    type Properties = TrailProperties;
    const TYPE: &'static str = "AWS::CloudTrail::Trail";
    fn properties(&self) -> &TrailProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TrailProperties {
        &mut self.properties
    }
}

impl From<TrailProperties> for Trail {
    fn from(properties: TrailProperties) -> Trail {
        Trail { properties }
    }
}

pub mod trail {
    #[derive(Serialize, Deserialize)]
    pub struct DataResource {
        #[serde(rename="Type")]
        pub type_: String,
        #[serde(rename="Values")]
        pub values: Vec<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct EventSelector {
        #[serde(rename="DataResources")]
        pub data_resources: Vec<DataResource>,
        #[serde(rename="IncludeManagementEvents")]
        pub include_management_events: bool,
        #[serde(rename="ReadWriteType")]
        pub read_write_type: String,
    }

}


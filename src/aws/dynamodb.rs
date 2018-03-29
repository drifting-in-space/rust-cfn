/// The [`AWS::DynamoDB::Table`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-table.html) resource.
pub struct Table {
    properties: TableProperties
}

/// Properties for the `Table` resource.
#[derive(Serialize, Deserialize)]
pub struct TableProperties {
    #[serde(rename="AttributeDefinitions")]
    pub attribute_definitions: Vec<self::table::AttributeDefinition>,
    #[serde(rename="GlobalSecondaryIndexes")]
    pub global_secondary_indexes: Vec<self::table::GlobalSecondaryIndex>,
    #[serde(rename="KeySchema")]
    pub key_schema: Vec<self::table::KeySchema>,
    #[serde(rename="LocalSecondaryIndexes")]
    pub local_secondary_indexes: Vec<self::table::LocalSecondaryIndex>,
    #[serde(rename="ProvisionedThroughput")]
    pub provisioned_throughput: self::table::ProvisionedThroughput,
    #[serde(rename="SSESpecification")]
    pub sse_specification: self::table::SSESpecification,
    #[serde(rename="StreamSpecification")]
    pub stream_specification: self::table::StreamSpecification,
    #[serde(rename="TableName")]
    pub table_name: String,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    #[serde(rename="TimeToLiveSpecification")]
    pub time_to_live_specification: self::table::TimeToLiveSpecification,
}

impl<'a> ::Resource<'a> for Table {
    type Properties = TableProperties;
    const TYPE: &'static str = "AWS::DynamoDB::Table";
    fn properties(&self) -> &TableProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TableProperties {
        &mut self.properties
    }
}

impl From<TableProperties> for Table {
    fn from(properties: TableProperties) -> Table {
        Table { properties }
    }
}

pub mod table {
    #[derive(Serialize, Deserialize)]
    pub struct AttributeDefinition {
        #[serde(rename="AttributeName")]
        pub attribute_name: String,
        #[serde(rename="AttributeType")]
        pub attribute_type: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct GlobalSecondaryIndex {
        #[serde(rename="IndexName")]
        pub index_name: String,
        #[serde(rename="KeySchema")]
        pub key_schema: Vec<KeySchema>,
        #[serde(rename="Projection")]
        pub projection: Projection,
        #[serde(rename="ProvisionedThroughput")]
        pub provisioned_throughput: ProvisionedThroughput,
    }

    #[derive(Serialize, Deserialize)]
    pub struct KeySchema {
        #[serde(rename="AttributeName")]
        pub attribute_name: String,
        #[serde(rename="KeyType")]
        pub key_type: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct LocalSecondaryIndex {
        #[serde(rename="IndexName")]
        pub index_name: String,
        #[serde(rename="KeySchema")]
        pub key_schema: Vec<KeySchema>,
        #[serde(rename="Projection")]
        pub projection: Projection,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Projection {
        #[serde(rename="NonKeyAttributes")]
        pub non_key_attributes: Vec<String>,
        #[serde(rename="ProjectionType")]
        pub projection_type: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ProvisionedThroughput {
        #[serde(rename="ReadCapacityUnits")]
        pub read_capacity_units: u64,
        #[serde(rename="WriteCapacityUnits")]
        pub write_capacity_units: u64,
    }

    #[derive(Serialize, Deserialize)]
    pub struct SSESpecification {
        #[serde(rename="SSEEnabled")]
        pub sse_enabled: bool,
    }

    #[derive(Serialize, Deserialize)]
    pub struct StreamSpecification {
        #[serde(rename="StreamViewType")]
        pub stream_view_type: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct TimeToLiveSpecification {
        #[serde(rename="AttributeName")]
        pub attribute_name: String,
        #[serde(rename="Enabled")]
        pub enabled: bool,
    }

}


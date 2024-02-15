//! Types for the `FIS` service.

/// The [`AWS::FIS::ExperimentTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fis-experimenttemplate.html) resource type.
#[derive(Debug, Default)]
pub struct ExperimentTemplate {
    properties: ExperimentTemplateProperties
}

/// Properties for the `ExperimentTemplate` resource.
#[derive(Debug, Default)]
pub struct ExperimentTemplateProperties {
    /// Property [`Actions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fis-experimenttemplate.html#cfn-fis-experimenttemplate-actions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub actions: Option<::ValueMap<self::experiment_template::ExperimentTemplateAction>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fis-experimenttemplate.html#cfn-fis-experimenttemplate-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: ::Value<String>,
    /// Property [`ExperimentOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fis-experimenttemplate.html#cfn-fis-experimenttemplate-experimentoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub experiment_options: Option<::Value<self::experiment_template::ExperimentTemplateExperimentOptions>>,
    /// Property [`LogConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fis-experimenttemplate.html#cfn-fis-experimenttemplate-logconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub log_configuration: Option<::Value<self::experiment_template::ExperimentTemplateLogConfiguration>>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fis-experimenttemplate.html#cfn-fis-experimenttemplate-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`StopConditions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fis-experimenttemplate.html#cfn-fis-experimenttemplate-stopconditions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub stop_conditions: ::ValueList<self::experiment_template::ExperimentTemplateStopCondition>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fis-experimenttemplate.html#cfn-fis-experimenttemplate-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: ::ValueMap<String>,
    /// Property [`Targets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fis-experimenttemplate.html#cfn-fis-experimenttemplate-targets).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub targets: ::ValueMap<self::experiment_template::ExperimentTemplateTarget>,
}

impl ::serde::Serialize for ExperimentTemplateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref actions) = self.actions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Actions", actions)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        if let Some(ref experiment_options) = self.experiment_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExperimentOptions", experiment_options)?;
        }
        if let Some(ref log_configuration) = self.log_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogConfiguration", log_configuration)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StopConditions", &self.stop_conditions)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Targets", &self.targets)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ExperimentTemplateProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ExperimentTemplateProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ExperimentTemplateProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ExperimentTemplateProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut actions: Option<::ValueMap<self::experiment_template::ExperimentTemplateAction>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut experiment_options: Option<::Value<self::experiment_template::ExperimentTemplateExperimentOptions>> = None;
                let mut log_configuration: Option<::Value<self::experiment_template::ExperimentTemplateLogConfiguration>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut stop_conditions: Option<::ValueList<self::experiment_template::ExperimentTemplateStopCondition>> = None;
                let mut tags: Option<::ValueMap<String>> = None;
                let mut targets: Option<::ValueMap<self::experiment_template::ExperimentTemplateTarget>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Actions" => {
                            actions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExperimentOptions" => {
                            experiment_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LogConfiguration" => {
                            log_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StopConditions" => {
                            stop_conditions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Targets" => {
                            targets = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ExperimentTemplateProperties {
                    actions: actions,
                    description: description.ok_or(::serde::de::Error::missing_field("Description"))?,
                    experiment_options: experiment_options,
                    log_configuration: log_configuration,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    stop_conditions: stop_conditions.ok_or(::serde::de::Error::missing_field("StopConditions"))?,
                    tags: tags.ok_or(::serde::de::Error::missing_field("Tags"))?,
                    targets: targets.ok_or(::serde::de::Error::missing_field("Targets"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ExperimentTemplate {
    type Properties = ExperimentTemplateProperties;
    const TYPE: &'static str = "AWS::FIS::ExperimentTemplate";
    fn properties(&self) -> &ExperimentTemplateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ExperimentTemplateProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ExperimentTemplate {}

impl From<ExperimentTemplateProperties> for ExperimentTemplate {
    fn from(properties: ExperimentTemplateProperties) -> ExperimentTemplate {
        ExperimentTemplate { properties }
    }
}

/// The [`AWS::FIS::TargetAccountConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fis-targetaccountconfiguration.html) resource type.
#[derive(Debug, Default)]
pub struct TargetAccountConfiguration {
    properties: TargetAccountConfigurationProperties
}

/// Properties for the `TargetAccountConfiguration` resource.
#[derive(Debug, Default)]
pub struct TargetAccountConfigurationProperties {
    /// Property [`AccountId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fis-targetaccountconfiguration.html#cfn-fis-targetaccountconfiguration-accountid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub account_id: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fis-targetaccountconfiguration.html#cfn-fis-targetaccountconfiguration-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`ExperimentTemplateId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fis-targetaccountconfiguration.html#cfn-fis-targetaccountconfiguration-experimenttemplateid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub experiment_template_id: ::Value<String>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fis-targetaccountconfiguration.html#cfn-fis-targetaccountconfiguration-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
}

impl ::serde::Serialize for TargetAccountConfigurationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccountId", &self.account_id)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExperimentTemplateId", &self.experiment_template_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TargetAccountConfigurationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TargetAccountConfigurationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TargetAccountConfigurationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TargetAccountConfigurationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut account_id: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut experiment_template_id: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccountId" => {
                            account_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExperimentTemplateId" => {
                            experiment_template_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TargetAccountConfigurationProperties {
                    account_id: account_id.ok_or(::serde::de::Error::missing_field("AccountId"))?,
                    description: description,
                    experiment_template_id: experiment_template_id.ok_or(::serde::de::Error::missing_field("ExperimentTemplateId"))?,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for TargetAccountConfiguration {
    type Properties = TargetAccountConfigurationProperties;
    const TYPE: &'static str = "AWS::FIS::TargetAccountConfiguration";
    fn properties(&self) -> &TargetAccountConfigurationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TargetAccountConfigurationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for TargetAccountConfiguration {}

impl From<TargetAccountConfigurationProperties> for TargetAccountConfiguration {
    fn from(properties: TargetAccountConfigurationProperties) -> TargetAccountConfiguration {
        TargetAccountConfiguration { properties }
    }
}

pub mod experiment_template {
    //! Property types for the `ExperimentTemplate` resource.

    /// The [`AWS::FIS::ExperimentTemplate.CloudWatchLogsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-cloudwatchlogsconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct CloudWatchLogsConfiguration {
        /// Property [`LogGroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-cloudwatchlogsconfiguration.html#cfn-fis-experimenttemplate-cloudwatchlogsconfiguration-loggrouparn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_group_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for CloudWatchLogsConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogGroupArn", &self.log_group_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CloudWatchLogsConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudWatchLogsConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CloudWatchLogsConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CloudWatchLogsConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut log_group_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LogGroupArn" => {
                                log_group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CloudWatchLogsConfiguration {
                        log_group_arn: log_group_arn.ok_or(::serde::de::Error::missing_field("LogGroupArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FIS::ExperimentTemplate.ExperimentTemplateAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplateaction.html) property type.
    #[derive(Debug, Default)]
    pub struct ExperimentTemplateAction {
        /// Property [`ActionId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplateaction.html#cfn-fis-experimenttemplate-experimenttemplateaction-actionid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action_id: ::Value<String>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplateaction.html#cfn-fis-experimenttemplate-experimenttemplateaction-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplateaction.html#cfn-fis-experimenttemplate-experimenttemplateaction-parameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameters: Option<::ValueMap<String>>,
        /// Property [`StartAfter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplateaction.html#cfn-fis-experimenttemplate-experimenttemplateaction-startafter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start_after: Option<::ValueList<String>>,
        /// Property [`Targets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplateaction.html#cfn-fis-experimenttemplate-experimenttemplateaction-targets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub targets: Option<::ValueMap<String>>,
    }

    impl ::codec::SerializeValue for ExperimentTemplateAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ActionId", &self.action_id)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref parameters) = self.parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
            }
            if let Some(ref start_after) = self.start_after {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartAfter", start_after)?;
            }
            if let Some(ref targets) = self.targets {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Targets", targets)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ExperimentTemplateAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ExperimentTemplateAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ExperimentTemplateAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ExperimentTemplateAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut action_id: Option<::Value<String>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut parameters: Option<::ValueMap<String>> = None;
                    let mut start_after: Option<::ValueList<String>> = None;
                    let mut targets: Option<::ValueMap<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ActionId" => {
                                action_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Parameters" => {
                                parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartAfter" => {
                                start_after = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Targets" => {
                                targets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ExperimentTemplateAction {
                        action_id: action_id.ok_or(::serde::de::Error::missing_field("ActionId"))?,
                        description: description,
                        parameters: parameters,
                        start_after: start_after,
                        targets: targets,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FIS::ExperimentTemplate.ExperimentTemplateExperimentOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplateexperimentoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct ExperimentTemplateExperimentOptions {
        /// Property [`AccountTargeting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplateexperimentoptions.html#cfn-fis-experimenttemplate-experimenttemplateexperimentoptions-accounttargeting).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub account_targeting: Option<::Value<String>>,
        /// Property [`EmptyTargetResolutionMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplateexperimentoptions.html#cfn-fis-experimenttemplate-experimenttemplateexperimentoptions-emptytargetresolutionmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub empty_target_resolution_mode: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ExperimentTemplateExperimentOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref account_targeting) = self.account_targeting {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccountTargeting", account_targeting)?;
            }
            if let Some(ref empty_target_resolution_mode) = self.empty_target_resolution_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmptyTargetResolutionMode", empty_target_resolution_mode)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ExperimentTemplateExperimentOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ExperimentTemplateExperimentOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ExperimentTemplateExperimentOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ExperimentTemplateExperimentOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut account_targeting: Option<::Value<String>> = None;
                    let mut empty_target_resolution_mode: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccountTargeting" => {
                                account_targeting = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EmptyTargetResolutionMode" => {
                                empty_target_resolution_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ExperimentTemplateExperimentOptions {
                        account_targeting: account_targeting,
                        empty_target_resolution_mode: empty_target_resolution_mode,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FIS::ExperimentTemplate.ExperimentTemplateLogConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplatelogconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ExperimentTemplateLogConfiguration {
        /// Property [`CloudWatchLogsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplatelogconfiguration.html#cfn-fis-experimenttemplate-experimenttemplatelogconfiguration-cloudwatchlogsconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_logs_configuration: Option<::Value<CloudWatchLogsConfiguration>>,
        /// Property [`LogSchemaVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplatelogconfiguration.html#cfn-fis-experimenttemplate-experimenttemplatelogconfiguration-logschemaversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_schema_version: ::Value<u32>,
        /// Property [`S3Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplatelogconfiguration.html#cfn-fis-experimenttemplate-experimenttemplatelogconfiguration-s3configuration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_configuration: Option<::Value<S3Configuration>>,
    }

    impl ::codec::SerializeValue for ExperimentTemplateLogConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cloud_watch_logs_configuration) = self.cloud_watch_logs_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLogsConfiguration", cloud_watch_logs_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogSchemaVersion", &self.log_schema_version)?;
            if let Some(ref s3_configuration) = self.s3_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Configuration", s3_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ExperimentTemplateLogConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ExperimentTemplateLogConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ExperimentTemplateLogConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ExperimentTemplateLogConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloud_watch_logs_configuration: Option<::Value<CloudWatchLogsConfiguration>> = None;
                    let mut log_schema_version: Option<::Value<u32>> = None;
                    let mut s3_configuration: Option<::Value<S3Configuration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudWatchLogsConfiguration" => {
                                cloud_watch_logs_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogSchemaVersion" => {
                                log_schema_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Configuration" => {
                                s3_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ExperimentTemplateLogConfiguration {
                        cloud_watch_logs_configuration: cloud_watch_logs_configuration,
                        log_schema_version: log_schema_version.ok_or(::serde::de::Error::missing_field("LogSchemaVersion"))?,
                        s3_configuration: s3_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FIS::ExperimentTemplate.ExperimentTemplateStopCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplatestopcondition.html) property type.
    #[derive(Debug, Default)]
    pub struct ExperimentTemplateStopCondition {
        /// Property [`Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplatestopcondition.html#cfn-fis-experimenttemplate-experimenttemplatestopcondition-source).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplatestopcondition.html#cfn-fis-experimenttemplate-experimenttemplatestopcondition-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ExperimentTemplateStopCondition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Source", &self.source)?;
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ExperimentTemplateStopCondition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ExperimentTemplateStopCondition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ExperimentTemplateStopCondition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ExperimentTemplateStopCondition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut source: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Source" => {
                                source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ExperimentTemplateStopCondition {
                        source: source.ok_or(::serde::de::Error::missing_field("Source"))?,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FIS::ExperimentTemplate.ExperimentTemplateTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplatetarget.html) property type.
    #[derive(Debug, Default)]
    pub struct ExperimentTemplateTarget {
        /// Property [`Filters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplatetarget.html#cfn-fis-experimenttemplate-experimenttemplatetarget-filters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filters: Option<::ValueList<ExperimentTemplateTargetFilter>>,
        /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplatetarget.html#cfn-fis-experimenttemplate-experimenttemplatetarget-parameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameters: Option<::ValueMap<String>>,
        /// Property [`ResourceArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplatetarget.html#cfn-fis-experimenttemplate-experimenttemplatetarget-resourcearns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_arns: Option<::ValueList<String>>,
        /// Property [`ResourceTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplatetarget.html#cfn-fis-experimenttemplate-experimenttemplatetarget-resourcetags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_tags: Option<::ValueMap<String>>,
        /// Property [`ResourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplatetarget.html#cfn-fis-experimenttemplate-experimenttemplatetarget-resourcetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_type: ::Value<String>,
        /// Property [`SelectionMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplatetarget.html#cfn-fis-experimenttemplate-experimenttemplatetarget-selectionmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub selection_mode: ::Value<String>,
    }

    impl ::codec::SerializeValue for ExperimentTemplateTarget {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref filters) = self.filters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Filters", filters)?;
            }
            if let Some(ref parameters) = self.parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
            }
            if let Some(ref resource_arns) = self.resource_arns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceArns", resource_arns)?;
            }
            if let Some(ref resource_tags) = self.resource_tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceTags", resource_tags)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceType", &self.resource_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SelectionMode", &self.selection_mode)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ExperimentTemplateTarget {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ExperimentTemplateTarget, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ExperimentTemplateTarget;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ExperimentTemplateTarget")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut filters: Option<::ValueList<ExperimentTemplateTargetFilter>> = None;
                    let mut parameters: Option<::ValueMap<String>> = None;
                    let mut resource_arns: Option<::ValueList<String>> = None;
                    let mut resource_tags: Option<::ValueMap<String>> = None;
                    let mut resource_type: Option<::Value<String>> = None;
                    let mut selection_mode: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Filters" => {
                                filters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Parameters" => {
                                parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceArns" => {
                                resource_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceTags" => {
                                resource_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceType" => {
                                resource_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SelectionMode" => {
                                selection_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ExperimentTemplateTarget {
                        filters: filters,
                        parameters: parameters,
                        resource_arns: resource_arns,
                        resource_tags: resource_tags,
                        resource_type: resource_type.ok_or(::serde::de::Error::missing_field("ResourceType"))?,
                        selection_mode: selection_mode.ok_or(::serde::de::Error::missing_field("SelectionMode"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FIS::ExperimentTemplate.ExperimentTemplateTargetFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplatetargetfilter.html) property type.
    #[derive(Debug, Default)]
    pub struct ExperimentTemplateTargetFilter {
        /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplatetargetfilter.html#cfn-fis-experimenttemplate-experimenttemplatetargetfilter-path).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path: ::Value<String>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplatetargetfilter.html#cfn-fis-experimenttemplate-experimenttemplatetargetfilter-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for ExperimentTemplateTargetFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", &self.path)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", &self.values)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ExperimentTemplateTargetFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ExperimentTemplateTargetFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ExperimentTemplateTargetFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ExperimentTemplateTargetFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut path: Option<::Value<String>> = None;
                    let mut values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Path" => {
                                path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ExperimentTemplateTargetFilter {
                        path: path.ok_or(::serde::de::Error::missing_field("Path"))?,
                        values: values.ok_or(::serde::de::Error::missing_field("Values"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FIS::ExperimentTemplate.S3Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-s3configuration.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Configuration {
        /// Property [`BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-s3configuration.html#cfn-fis-experimenttemplate-s3configuration-bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_name: ::Value<String>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-s3configuration.html#cfn-fis-experimenttemplate-s3configuration-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3Configuration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", &self.bucket_name)?;
            if let Some(ref prefix) = self.prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", prefix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Configuration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Configuration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Configuration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Configuration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_name: Option<::Value<String>> = None;
                    let mut prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketName" => {
                                bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Configuration {
                        bucket_name: bucket_name.ok_or(::serde::de::Error::missing_field("BucketName"))?,
                        prefix: prefix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
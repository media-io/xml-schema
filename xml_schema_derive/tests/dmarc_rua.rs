use xml_schema_derive::XmlSchema;
use yaserde::de::from_str;

#[test]
fn dmarc_rua_string() {
  #[derive(Debug, XmlSchema)]
  #[xml_schema(source = "xml_schema_derive/tests/dmarc_rua.xsd", target_prefix = "dmarc")]
  struct DmarcRuaSchema;

  let xml_1 = r#"<?xml version="1.0" encoding="UTF-8" ?>
<feedback>
  <report_metadata>
    <org_name>google.com</org_name>
    <email>noreply-dmarc-support@google.com</email>
    <extra_contact_info>https://support.google.com/a/answer/2466580</extra_contact_info>
    <report_id>5717107811868587391</report_id>
    <date_range>
      <begin>1706832000</begin>
      <end>1706918399</end>
    </date_range>
  </report_metadata>
  <policy_published>
    <domain>example.com</domain>
    <adkim>r</adkim>
    <aspf>r</aspf>
    <p>none</p>
    <sp>none</sp>
    <pct>100</pct>
    <np>none</np>
  </policy_published>
  <record>
    <row>
      <source_ip>185.70.43.17</source_ip>
      <count>1</count>
      <policy_evaluated>
        <disposition>none</disposition>
        <dkim>pass</dkim>
        <spf>pass</spf>
      </policy_evaluated>
    </row>
    <identifiers>
      <header_from>example.com</header_from>
    </identifiers>
    <auth_results>
      <dkim>
        <domain>example.com</domain>
        <result>pass</result>
        <selector>protonmail2</selector>
      </dkim>
      <spf>
        <domain>example.com</domain>
        <result>pass</result>
      </spf>
    </auth_results>
  </record>
</feedback>
  "#;

  let _: Feedback = from_str(xml_1).unwrap();
}

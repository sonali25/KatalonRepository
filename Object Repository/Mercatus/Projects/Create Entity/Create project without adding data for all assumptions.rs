<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Create project without adding data for all assumptions</description>
   <name>Create project without adding data for all assumptions</name>
   <tag></tag>
   <elementGuidId>e4a517e9-acd3-46cc-bbd8-de9de2d08561</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\&quot;shortName\&quot;:\&quot;Project_incorrect_assumption\&quot;,\n\&quot;tzOffsetPlanned\&quot;:330,\n\&quot;category\&quot;:\&quot;Project\&quot;,\n\&quot;organizationId\&quot;:${\&quot;OrgId\&quot;},\n\&quot;taskTemplateId\&quot;:\&quot;\&quot;,\n\&quot;autoSelectTemplateFlag\&quot;:false,\n\&quot;projectFieldMap\&quot;:\n  {\&quot;25034\&quot;:\&quot;test\&quot;,\n   \&quot;25036\&quot;:\&quot;3\&quot;,\n   \&quot;25038\&quot;:\&quot;\&quot;,\n   \&quot;25040\&quot;:\&quot;3291\&quot;,\n   \&quot;25041\&quot;:\&quot;03/11/2019\&quot;,\n   \&quot;25042\&quot;:\&quot;Yes\&quot;,\n   \&quot;25056\&quot;:\&quot;Project\&quot;,\n   \&quot;26376\&quot;:\&quot;\&quot;\n   }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${Authorization}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url3}/rest/projects?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.Authorization</defaultValue>
      <description></description>
      <id>c35a4655-9467-4db5-a414-c14afcc6643a</id>
      <masked>false</masked>
      <name>Authorization</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Create Entity').getValue(1, 1)</defaultValue>
      <description></description>
      <id>98a6a9f2-0bf1-4297-bad5-f5c33ea845fa</id>
      <masked>false</masked>
      <name>OrgId</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Create Entity').getValue(1, 1)</defaultValue>
      <description></description>
      <id>a8c41223-63fe-4b23-9a9c-c9752bfd892a</id>
      <masked>false</masked>
      <name>url3</name>
   </variables>
   <variables>
      <defaultValue>'36505675'</defaultValue>
      <description></description>
      <id>5d07d589-83c9-4d29-bec1-5dd6330261be</id>
      <masked>false</masked>
      <name>testrail_tc_id</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

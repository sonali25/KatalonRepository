<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Create entity without creation fields in API body</description>
   <name>Create entity without creation fields in API body</name>
   <tag></tag>
   <elementGuidId>06eb49fe-789f-4d81-88a6-e6bf234b88dc</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\&quot;shortName\&quot;:\&quot;\&quot;,\n\&quot;tzOffsetPlanned\&quot;:330,\n\&quot;category\&quot;:\&quot;Project\&quot;,\n\&quot;organizationId\&quot;:\&quot;${OrgId}\&quot;,\n\&quot;taskTemplateId\&quot;:\&quot;\&quot;,\n\&quot;autoSelectTemplateFlag\&quot;:false,\n\&quot;projectFieldMap\&quot;:\n  {\n   }\n}&quot;,
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
      <defaultValue>findTestData('Create Entity').getValue(1, 1)</defaultValue>
      <description></description>
      <id>98a4c641-6ae9-4cb8-abc4-c50318aa6ce3</id>
      <masked>false</masked>
      <name>OrgId</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Create Entity').getValue(1, 1)</defaultValue>
      <description></description>
      <id>61540bf0-6c66-40df-b884-0c17c67fb856</id>
      <masked>false</masked>
      <name>url3</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Authorization</defaultValue>
      <description></description>
      <id>bbd952a4-768a-4e79-9f4d-de255175baf8</id>
      <masked>false</masked>
      <name>Authorization</name>
   </variables>
   <variables>
      <defaultValue>'36505677'</defaultValue>
      <description></description>
      <id>b040f169-259d-43b4-981b-f9ea240284a3</id>
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

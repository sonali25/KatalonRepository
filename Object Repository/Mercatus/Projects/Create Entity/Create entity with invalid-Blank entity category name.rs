<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Create entity with invalid/Blank entity category name</description>
   <name>Create entity with invalid-Blank entity category name</name>
   <tag></tag>
   <elementGuidId>65dd2c4a-c930-445b-b4f8-3463d0bb4668</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\&quot;shortName\&quot;:\&quot;Project_Blank_Category_Name\&quot;,\n\&quot;tzOffsetPlanned\&quot;:330,\n\&quot;category\&quot;:\&quot;\&quot;,\n\&quot;organizationId\&quot;:\&quot;${OrgId}\&quot;,\n\&quot;taskTemplateId\&quot;:\&quot;\&quot;,\n\&quot;autoSelectTemplateFlag\&quot;:false,\n\&quot;projectFieldMap\&quot;:\n  {\&quot;25034\&quot;:\&quot;test\&quot;,\n   \&quot;25036\&quot;:\&quot;4\&quot;,\n   \&quot;25038\&quot;:\&quot;b\&quot;,\n   \&quot;25040\&quot;:\&quot;3291\&quot;,\n   \&quot;25041\&quot;:\&quot;03/11/2019\&quot;,\n   \&quot;25042\&quot;:\&quot;Yes\&quot;,\n   \&quot;25056\&quot;:\&quot;Project\&quot;,\n   \&quot;26376\&quot;:\&quot;2\&quot;\n   }\n}&quot;,
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
      <id>ede16af9-0cf4-4d29-94df-7864518a36b0</id>
      <masked>false</masked>
      <name>OrgId</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Create Entity').getValue(1, 1)</defaultValue>
      <description></description>
      <id>9932acc1-02ff-44f0-9aea-842771eba59c</id>
      <masked>false</masked>
      <name>url3</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Authorization</defaultValue>
      <description></description>
      <id>3ce7adaa-51e8-4c66-a3a6-48204558ea9f</id>
      <masked>false</masked>
      <name>Authorization</name>
   </variables>
   <variables>
      <defaultValue>'36505678'</defaultValue>
      <description></description>
      <id>7c888617-d544-4775-b581-019f698e05ab</id>
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

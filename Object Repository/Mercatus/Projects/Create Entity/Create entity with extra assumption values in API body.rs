<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Create entity with extra assumption values in API body</description>
   <name>Create entity with extra assumption values in API body</name>
   <tag></tag>
   <elementGuidId>32ebca22-c396-4d6a-881e-0606bde7b079</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\&quot;shortName\&quot;:\&quot;Project_Extra_Assumption_Body\&quot;,\n\&quot;tzOffsetPlanned\&quot;:330,\n\&quot;category\&quot;:\&quot;\&quot;,\n\&quot;organizationId\&quot;:\&quot;${OrgId}\&quot;,\n\&quot;taskTemplateId\&quot;:\&quot;\&quot;,\n\&quot;autoSelectTemplateFlag\&quot;:false,\n\&quot;projectFieldMap\&quot;:\n  {\&quot;25034\&quot;:\&quot;test\&quot;,\n   \&quot;25036\&quot;:\&quot;4\&quot;,\n   \&quot;25038\&quot;:\&quot;b\&quot;,\n   \&quot;25040\&quot;:\&quot;3291\&quot;,\n   \&quot;25041\&quot;:\&quot;03/11/2019\&quot;,\n   \&quot;25042\&quot;:\&quot;Yes\&quot;,\n   \&quot;25056\&quot;:\&quot;Project\&quot;,\n   \&quot;26376\&quot;:\&quot;2\&quot;,\n   \&quot;25003\&quot;:\&quot;0.25\&quot;,\n   \&quot;25000\&quot;:\&quot;Testing\&quot;\n   }\n}&quot;,
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
      <id>e99be0e3-4057-49f7-8f23-187b6b8297fe</id>
      <masked>false</masked>
      <name>OrgId</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Create Entity').getValue(1, 1)</defaultValue>
      <description></description>
      <id>8e597ea5-9b03-4481-a1db-a4105a9b821c</id>
      <masked>false</masked>
      <name>url3</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Authorization</defaultValue>
      <description></description>
      <id>3958e526-a5df-4cb6-806b-b035a2d774fc</id>
      <masked>false</masked>
      <name>Authorization</name>
   </variables>
   <variables>
      <defaultValue>'36505672'</defaultValue>
      <description></description>
      <id>a15311d6-5b21-4a6b-8a05-c45cab2eecfe</id>
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

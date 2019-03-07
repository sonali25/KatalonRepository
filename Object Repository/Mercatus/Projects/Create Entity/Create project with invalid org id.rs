<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Create project with invalid org id</description>
   <name>Create project with invalid org id</name>
   <tag></tag>
   <elementGuidId>4e43cbab-ea6e-4ac5-8229-4b7d728b662b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\&quot;shortName\&quot;:\&quot;Project_Incorrect_OrgId\&quot;,\n\&quot;tzOffsetPlanned\&quot;:330,\n\&quot;category\&quot;:\&quot;Project\&quot;,\n\&quot;organizationId\&quot;:10,\n\&quot;taskTemplateId\&quot;:\&quot;\&quot;,\n\&quot;autoSelectTemplateFlag\&quot;:false,\n\&quot;projectFieldMap\&quot;:\n  {\&quot;25034\&quot;:\&quot;test\&quot;,\n   \&quot;25036\&quot;:\&quot;3\&quot;,\n   \&quot;25038\&quot;:\&quot;a\&quot;,\n   \&quot;25040\&quot;:\&quot;3291\&quot;,\n   \&quot;25041\&quot;:\&quot;03/11/2019\&quot;,\n   \&quot;25042\&quot;:\&quot;Yes\&quot;,\n   \&quot;25056\&quot;:\&quot;Project\&quot;,\n   \&quot;26376\&quot;:\&quot;2\&quot;\n   }\n}&quot;,
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
      <id>0030de19-e0f7-4eb4-b9d2-a2a77b0ca2f7</id>
      <masked>false</masked>
      <name>Authorization</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Create Entity').getValue(1, 1)</defaultValue>
      <description></description>
      <id>62d8f17d-93da-4f36-a27f-076af135410e</id>
      <masked>false</masked>
      <name>url3</name>
   </variables>
   <variables>
      <defaultValue>'36505676'</defaultValue>
      <description></description>
      <id>8f2f90d9-b06e-49ee-baa0-112feb1787e3</id>
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

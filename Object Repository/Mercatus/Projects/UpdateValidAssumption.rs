<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Update Assumption value of project</description>
   <name>UpdateValidAssumption</name>
   <tag></tag>
   <elementGuidId>db71d5e3-dfee-4240-a352-c0ef7ca4196e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\&quot;id\&quot;: \&quot;${AssumpId}\&quot;,\n\&quot;oldValue\&quot;: null,\n\&quot;projectId\&quot;: \&quot;30684\&quot;,\n\&quot;value\&quot;: \&quot;2\&quot;\n}&quot;,
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
      <value>Basic YWJjQGV4YW1wbGUuY29tOlRlc3RAMTIzNA==</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://us-staging3.gomercatus.com/rest/projectDetail/updateAssumption?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>findTestData('CreateProject2').getValue(4, 1)</defaultValue>
      <description></description>
      <id>0bad5093-4d5a-4dd0-935f-836a67fb685b</id>
      <masked>false</masked>
      <name>Projectid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.AssumpId</defaultValue>
      <description></description>
      <id>effdfb5c-753e-4d39-a899-7c67f068be4f</id>
      <masked>false</masked>
      <name>AssumpId</name>
   </variables>
   <variables>
      <defaultValue>1900</defaultValue>
      <description></description>
      <id>36e5e694-024e-4511-a540-9996faa409a9</id>
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






WS.verifyElementPropertyValue(response, 'issues[0].fields.project.key', 'KTP')

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

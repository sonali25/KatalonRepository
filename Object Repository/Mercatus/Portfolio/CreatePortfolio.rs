<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Creates portfolio</description>
   <name>CreatePortfolio</name>
   <tag></tag>
   <elementGuidId>140e4c1d-d87f-4401-822d-cf205d354070</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\&quot;category\&quot;:${category},\n\&quot;name\&quot;: ${name},\n\&quot;orgId\&quot;: ${orgId},\n\&quot;projectList\&quot;: ${projectList}\n}&quot;,
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
      <value>Basic bWVyY2F0dXNAc3luZXJ6aXAuY29tOlRlc3RAMTIzNDU=</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url1}/rest/projectPortfolio?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>findTestData('Variables').getValue(1, 1)</defaultValue>
      <description></description>
      <id>6435a0d9-b172-4192-9e84-404c364a1028</id>
      <masked>false</masked>
      <name>url1</name>
   </variables>
   <variables>
      <defaultValue>findTestData('CreateProject2').getValue(1, 1)</defaultValue>
      <description></description>
      <id>962b49de-3d84-45f2-a36c-d13743c9b64d</id>
      <masked>false</masked>
      <name>categroy</name>
   </variables>
   <variables>
      <defaultValue>findTestData('CreateProject2').getValue(2, 1)</defaultValue>
      <description></description>
      <id>f94eca43-3218-4dc6-b617-11beff3e68f7</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <variables>
      <defaultValue>findTestData('CreateProject2').getValue(3, 1)</defaultValue>
      <description></description>
      <id>8f44f018-ee78-4695-8218-aecad335210e</id>
      <masked>false</masked>
      <name>orgId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.projId</defaultValue>
      <description></description>
      <id>c42061d7-a560-4021-9e02-ff2274c778b1</id>
      <masked>false</masked>
      <name>projectList</name>
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

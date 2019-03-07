<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreateUser</name>
   <tag></tag>
   <elementGuidId>4061a838-70eb-49c1-99bc-1d5d958880d8</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;orgId\&quot;: 818,\n  \&quot;firstName\&quot;: \&quot;${firstname}\&quot;,\n  \&quot;lastName\&quot;: \&quot;S\&quot;,\n  \&quot;email\&quot;: \&quot;s1o2me42@example.com\&quot;,\n  \&quot;contactNumber\&quot;: \&quot;\&quot;,\n  \&quot;encryptedPassword\&quot;: \&quot;Test@1234\&quot;,\n  \&quot;projectAccessInd\&quot;: false,\n  \&quot;licenseType\&quot;: \&quot;Power\&quot;,\n  \&quot;passwordExpiry\&quot;: \&quot;0\&quot;,\n  \&quot;notificationInd\&quot;: \&quot;true\&quot;,\n  \&quot;notificationType\&quot;: \&quot;0\&quot;\n}&quot;,
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
   <restUrl>${url2}/rest/users?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>findTestData('Variables').getValue(1, 1)</defaultValue>
      <description></description>
      <id>68706911-ff70-4ded-b4ee-d000118b0d69</id>
      <masked>false</masked>
      <name>url2</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Variables').getValue(2, 1)</defaultValue>
      <description></description>
      <id>f0569d3f-b96e-41c2-8ebc-a9fb20b73825</id>
      <masked>false</masked>
      <name>org</name>
   </variables>
   <variables>
      <defaultValue>findTestData('CreateProject2').getValue(2, 1)</defaultValue>
      <description></description>
      <id>695155a3-21f0-433c-bfa2-9940ad9d0240</id>
      <masked>false</masked>
      <name>firstname</name>
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

<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Provide invalid id of project while creating portfolio</description>
   <name>CreatePortfolioWithInvalid Id</name>
   <tag></tag>
   <elementGuidId>0aa8fe26-95b3-4efd-9457-5fb8f25828dc</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\&quot;category\&quot;:\&quot;Project\&quot;,\n\&quot;name\&quot;: \&quot;Jan 14\&quot;,\n\&quot;orgId\&quot;: ${org},\n\&quot;projectList\&quot;: [\&quot;1\&quot;]\n}&quot;,
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
   <restUrl>${url4}/rest/projectPortfolio?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>findTestData('Variables').getValue(1, 1)</defaultValue>
      <description></description>
      <id>677e2904-bca8-460e-b3a3-18eab7423759</id>
      <masked>false</masked>
      <name>url4</name>
   </variables>
   <variables>
      <defaultValue>findTestData('CreateProject2').getValue(3, 1)</defaultValue>
      <description></description>
      <id>f2cbff06-6a41-46bf-9e94-0915810fad7a</id>
      <masked>false</masked>
      <name>org</name>
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

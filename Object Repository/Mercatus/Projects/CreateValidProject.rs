<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreateValidProject</name>
   <tag></tag>
   <elementGuidId>6d52a4df-7386-42e3-85ab-cb9dbcf4ddee</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;autoSelectTemplateFlag\&quot;: \&quot;false\&quot;\n\t\&quot;category\&quot;: \&quot;Project\&quot;\n\t\&quot;organizationId\&quot;: 607\n\t\&quot;projectFieldMap\&quot;: \n\t[{\n  \t\&quot;24160\&quot;: \&quot;02/27/2019\&quot;, \n  \t\&quot;24161\&quot;: \&quot;Hydro\&quot;, \n  \t\&quot;24162\&quot;: \&quot;Yes\&quot;, \n  \t\&quot;24891\&quot;: \&quot;3\&quot;\n\t}],\n\t\&quot;shortName\&quot;: \&quot;Auto template selection-1\&quot;\n\t\&quot;taskTemplateId\&quot;: \&quot;\&quot;\n\t\&quot;tzOffsetPlanned\&quot;: 330\n}&quot;,
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
      <value>Basic bWVyYy5wcm9kLnZlcmZAZ21haWwuY29tOlRlc3RAMTIzNDU=</value>
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
      <defaultValue>findTestData('Variables').getValue(1, 1)</defaultValue>
      <description></description>
      <id>13fc2790-2255-44b0-b5eb-70b24a9a3f57</id>
      <masked>false</masked>
      <name>url3</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.ProjNameG</defaultValue>
      <description></description>
      <id>d3b02267-45a5-4256-806e-7e13dc551b5c</id>
      <masked>false</masked>
      <name>ProjName</name>
   </variables>
   <variables>
      <defaultValue>159837</defaultValue>
      <description></description>
      <id>134e1d8b-2c43-4c8b-8ec5-855c0f0e1858</id>
      <masked>false</masked>
      <name>testrail_tc_id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Authorization</defaultValue>
      <description></description>
      <id>256146a4-551d-4809-892a-5197807be151</id>
      <masked>false</masked>
      <name>Authorization</name>
   </variables>
   <variables>
      <defaultValue>1</defaultValue>
      <description></description>
      <id>036564e2-54de-49ae-9aa4-0921d53815c6</id>
      <masked>false</masked>
      <name>cnt</name>
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

for (cnt=1;cnt&lt;4;cnt++)
return cnt











</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

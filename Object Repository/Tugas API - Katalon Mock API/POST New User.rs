<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST New User</name>
   <tag></tag>
   <elementGuidId>66939319-8001-43e5-999d-ee632c0c4acd</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;first_name\&quot;: \&quot;${first_name}\&quot;,\n  \&quot;last_name\&quot;: \&quot;${last_name}\&quot;,\n  \&quot;username\&quot;: \&quot;${username}\&quot;,\n  \&quot;job_position\&quot;: \&quot;${job_position}\&quot;,\n  \&quot;job_level\&quot;: \&quot;${job_level}\&quot;,\n  \&quot;salary\&quot;: 8000000,\n  \&quot;work_duration\&quot;: 2\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>apikey</name>
      <type>Main</type>
      <value>${GlobalVariable.API_KEY}</value>
      <webElementGuid>2ef5457c-225c-47ce-9d51-42c77705f0d5</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>809035ca-9e44-4d8b-8bf9-2b7bb8273935</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.3.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.BASE_URL}/users</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>330ae820-4242-4b25-9687-6e0da06c1033</id>
      <name>User Schema Validation</name>
      <type>JSON_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>Resource/users_schema.json</data>
      <activate>true</activate>
   </validationSteps>
   <variables>
      <defaultValue>'ardy'</defaultValue>
      <description></description>
      <id>017a9f74-b5cb-4549-8f7c-c60648403d00</id>
      <masked>false</masked>
      <name>first_name</name>
   </variables>
   <variables>
      <defaultValue>'sw'</defaultValue>
      <description></description>
      <id>f347d1e2-1faf-4e14-a385-f73d39b58a2b</id>
      <masked>false</masked>
      <name>last_name</name>
   </variables>
   <variables>
      <defaultValue>'ardy_edit'</defaultValue>
      <description></description>
      <id>4ece1cf8-3570-4c95-9acc-8b1de7371a60</id>
      <masked>false</masked>
      <name>username</name>
   </variables>
   <variables>
      <defaultValue>'Quality Assurance'</defaultValue>
      <description></description>
      <id>760744be-77f9-4c88-8f60-15e9f14e08b7</id>
      <masked>false</masked>
      <name>job_position</name>
   </variables>
   <variables>
      <defaultValue>'Mid'</defaultValue>
      <description></description>
      <id>28f80637-f779-499d-ac37-eaddb04cb4c8</id>
      <masked>false</masked>
      <name>job_level</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>9295724e-9be2-49c2-86ce-a1d351b6ede3</id>
      <masked>false</masked>
      <name>salary</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>281c7be1-7397-4976-829d-941b429a0bc4</id>
      <masked>false</masked>
      <name>work_duration</name>
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


WS.verifyResponseStatusCode(response, 201)

assertThat(response.getStatusCode()).isEqualTo(201)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

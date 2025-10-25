<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>UPDATE User</name>
   <tag></tag>
   <elementGuidId>6fa3b217-b432-46fb-a479-3bcb457b736c</elementGuidId>
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
   <restRequestMethod>PATCH</restRequestMethod>
   <restUrl>${GlobalVariable.BASE_URL}/users?username=eq.${username}</restUrl>
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
      <defaultValue>'ardyasdadasd'</defaultValue>
      <description></description>
      <id>a5ca6e28-096a-4386-a216-6b170a376561</id>
      <masked>false</masked>
      <name>first_name</name>
   </variables>
   <variables>
      <defaultValue>'sw'</defaultValue>
      <description></description>
      <id>51a7eaa2-db62-45cf-bea2-069d4a10238e</id>
      <masked>false</masked>
      <name>last_name</name>
   </variables>
   <variables>
      <defaultValue>'ardy_4583'</defaultValue>
      <description></description>
      <id>f1e500ee-d0dc-4990-a624-0c087eed8b9e</id>
      <masked>false</masked>
      <name>username</name>
   </variables>
   <variables>
      <defaultValue>'asdsadas'</defaultValue>
      <description></description>
      <id>3e3d8198-ef92-4bbc-8593-0a2891694b2b</id>
      <masked>false</masked>
      <name>job_position</name>
   </variables>
   <variables>
      <defaultValue>'asdsadasa'</defaultValue>
      <description></description>
      <id>e4837812-516a-41c6-a4d1-ee02639f97a2</id>
      <masked>false</masked>
      <name>job_level</name>
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


WS.verifyResponseStatusCode(response, 204)

assertThat(response.getStatusCode()).isEqualTo(204)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

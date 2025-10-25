import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

// ==========================================
// 🔹 STEP 1: Generate Random Username
// ==========================================
def randomNum = new Random().nextInt(9000) + 1000

def randomUsername = "ardy_$randomNum"

println("✅ Random username generated: $randomUsername")

GlobalVariable.randomUsername = randomUsername

// ==========================================
// 🔹 STEP 2: POST New User
// ==========================================
def postResponse = WS.sendRequest(findTestObject('Tugas API - Katalon Mock API/POST New User', [('first_name') : 'ardy', ('last_name') : 'sw'
            , ('username') : randomUsername, ('job_position') : 'Quality Assurance', ('job_level') : 'Mid', ('salary') : 12000000
            , ('work_duration') : 3.5]))

WS.verifyResponseStatusCode(postResponse, 201)

println('✅ POST validation passed')

// ==========================================
// 🔹 STEP 3: UPDATE User using stored username
// ==========================================
def updateResponse = WS.sendRequest(findTestObject('Tugas API - Katalon Mock API/UPDATE User', [('first_name') : 'ardy_updated'
            , ('last_name') : 'sw', ('username') : GlobalVariable.randomUsername, ('job_position') : 'Senior QA Engineer'
            , ('job_level') : 'Senior']))

WS.verifyResponseStatusCode(updateResponse, 204)

println('✅ UPDATE validation passed')

// ==========================================
// 🔹 STEP 4: GET Single User (verify changes)
// ==========================================
def getResponse = WS.sendRequest(findTestObject('Tugas API - Katalon Mock API/GET Single User', [('username') : GlobalVariable.randomUsername]))

WS.verifyResponseStatusCode(getResponse, 200)

WS.verifyElementPropertyValue(getResponse, '[0].username', GlobalVariable.randomUsername)

WS.verifyElementPropertyValue(getResponse, '[0].first_name', 'ardy_updated')

println('✅ GET validation passed')

// ==========================================
// 🔹 STEP 5: DELETE User
// ==========================================
def deleteResponse = WS.sendRequest(findTestObject('Tugas API - Katalon Mock API/DELETE User', [('username') : GlobalVariable.randomUsername]))

WS.verifyResponseStatusCode(deleteResponse, 204)

println('✅ DELETE validation passed')


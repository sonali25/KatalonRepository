import com.kms.katalon.core.logging.KeywordLogger
import com.kms.katalon.core.exception.StepFailedException
import com.kms.katalon.core.reporting.ReportUtil
import com.kms.katalon.core.main.TestCaseMain
import com.kms.katalon.core.testdata.TestDataColumn
import groovy.lang.MissingPropertyException
import com.kms.katalon.core.testcase.TestCaseBinding
import com.kms.katalon.core.driver.internal.DriverCleanerCollector
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.configuration.RunConfiguration
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import internal.GlobalVariable as GlobalVariable

Map<String, String> suiteProperties = new HashMap<String, String>();


suiteProperties.put('id', 'Test Suites/New Test Suite')

suiteProperties.put('name', 'New Test Suite')

suiteProperties.put('description', '')
 

DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.webui.contribution.WebUiDriverCleaner())
DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.mobile.contribution.MobileDriverCleaner())
DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.cucumber.keyword.internal.CucumberDriverCleaner())



RunConfiguration.setExecutionSettingFile("E:\\test\\API-test\\Reports\\New Test Suite\\20190117_203428\\execution.properties")

TestCaseMain.beforeStart()

TestCaseMain.startTestSuite('Test Suites/New Test Suite', suiteProperties, [new TestCaseBinding('Test Cases/MercatusTestCases/PortfolioTestCase/CreatePortfolioWithInavlidProject', 'Test Cases/MercatusTestCases/PortfolioTestCase/CreatePortfolioWithInavlidProject',  null), new TestCaseBinding('Test Cases/MercatusTestCases/PortfolioTestCase/CreateValidPortFolio', 'Test Cases/MercatusTestCases/PortfolioTestCase/CreateValidPortFolio',  null), new TestCaseBinding('Test Cases/MercatusTestCases/ProjectTestCases/CreateValidProjectTestCase', 'Test Cases/MercatusTestCases/ProjectTestCases/CreateValidProjectTestCase',  null), new TestCaseBinding('Test Cases/MercatusTestCases/ProjectTestCases/UpdateValidAssumptionTestcase', 'Test Cases/MercatusTestCases/ProjectTestCases/UpdateValidAssumptionTestcase',  null), new TestCaseBinding('Test Cases/MercatusTestCases/UsersTestCase/CreateUser', 'Test Cases/MercatusTestCases/UsersTestCase/CreateUser',  null)])

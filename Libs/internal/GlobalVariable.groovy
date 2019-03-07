package internal

import com.kms.katalon.core.configuration.RunConfiguration
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase

/**
 * This class is generated automatically by Katalon Studio and should not be modified or deleted.
 */
public class GlobalVariable {
     
    /**
     * <p></p>
     */
    public static Object projId
     
    /**
     * <p></p>
     */
    public static Object G_run_testrail_tc_id
     
    /**
     * <p></p>
     */
    public static Object G_run_testrail_tc_status
     
    /**
     * <p></p>
     */
    public static Object G_testrail_run_id
     
    /**
     * <p></p>
     */
    public static Object Authorization
     

    static {
        def allVariables = [:]        
        allVariables.put('default', ['projId' : '', 'G_run_testrail_tc_id' : [], 'G_run_testrail_tc_status' : [], 'G_testrail_run_id' : '4594', 'Authorization' : 'Basic YXB1cnZhQHRlc3RxYTEuY29tOlRlc3RAMTIzNA=='])
        
        String profileName = RunConfiguration.getExecutionProfile()
        def selectedVariables = allVariables[profileName]
		
		for(object in selectedVariables){
			String overridingGlobalVariable = RunConfiguration.getOverridingGlobalVariable(object.key)
			if(overridingGlobalVariable != null){
				selectedVariables.put(object.key, overridingGlobalVariable)
			}
		}

        projId = selectedVariables["projId"]
        G_run_testrail_tc_id = selectedVariables["G_run_testrail_tc_id"]
        G_run_testrail_tc_status = selectedVariables["G_run_testrail_tc_status"]
        G_testrail_run_id = selectedVariables["G_testrail_run_id"]
        Authorization = selectedVariables["Authorization"]
        
    }
}


/**
 * This class is generated automatically by Katalon Studio and should not be modified or deleted.
 */

import java.lang.String


def static "Testrail.get_tests"(
    	String id	) {
    (new Testrail()).get_tests(
        	id)
}

def static "Testrail.update_run"(
    	String id	
     , 	String array	) {
    (new Testrail()).update_run(
        	id
         , 	array)
}

def static "Testrail.add_results"(
    	String id	
     , 	String request	) {
    (new Testrail()).add_results(
        	id
         , 	request)
}

var N = null;var sourcesIndex = {};
sourcesIndex["google_api_proto"] = {"name":"","dirs":[{"name":"ccc","dirs":[{"name":"hosted","dirs":[{"name":"marketplace","dirs":[{"name":"v2","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"google","dirs":[{"name":"actions","dirs":[{"name":"sdk","dirs":[{"name":"v2","dirs":[{"name":"conversation","files":["mod.rs"]},{"name":"interactionmodel","dirs":[{"name":"prompt","files":["mod.rs"]},{"name":"type","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"type","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"ads","dirs":[{"name":"admob","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"googleads","dirs":[{"name":"v10","dirs":[{"name":"common","files":["mod.rs"]},{"name":"enums","files":["mod.rs"]},{"name":"errors","files":["mod.rs"]},{"name":"resources","files":["mod.rs"]},{"name":"services","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"v7","dirs":[{"name":"common","files":["mod.rs"]},{"name":"enums","files":["mod.rs"]},{"name":"errors","files":["mod.rs"]},{"name":"resources","files":["mod.rs"]},{"name":"services","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"v8","dirs":[{"name":"common","files":["mod.rs"]},{"name":"enums","files":["mod.rs"]},{"name":"errors","files":["mod.rs"]},{"name":"resources","files":["mod.rs"]},{"name":"services","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"v9","dirs":[{"name":"common","files":["mod.rs"]},{"name":"enums","files":["mod.rs"]},{"name":"errors","files":["mod.rs"]},{"name":"resources","files":["mod.rs"]},{"name":"services","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"analytics","dirs":[{"name":"admin","dirs":[{"name":"v1alpha","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"data","dirs":[{"name":"v1beta","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"api","dirs":[{"name":"expr","dirs":[{"name":"conformance","dirs":[{"name":"v1alpha1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"v1alpha1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"servicecontrol","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v2","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"servicemanagement","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"serviceusage","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"appengine","dirs":[{"name":"legacy","files":["mod.rs"]},{"name":"logging","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"v1","files":["mod.rs"]},{"name":"v1beta","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"apps","dirs":[{"name":"alertcenter","dirs":[{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"drive","dirs":[{"name":"activity","dirs":[{"name":"v2","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"script","dirs":[{"name":"type","dirs":[{"name":"calendar","files":["mod.rs"]},{"name":"docs","files":["mod.rs"]},{"name":"drive","files":["mod.rs"]},{"name":"gmail","files":["mod.rs"]},{"name":"sheets","files":["mod.rs"]},{"name":"slides","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"area120","dirs":[{"name":"tables","dirs":[{"name":"v1alpha1","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"assistant","dirs":[{"name":"embedded","dirs":[{"name":"v1alpha1","files":["mod.rs"]},{"name":"v1alpha2","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"bigtable","dirs":[{"name":"admin","dirs":[{"name":"v2","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"v2","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"bytestream","files":["mod.rs"]},{"name":"chromeos","dirs":[{"name":"moblab","dirs":[{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"uidetection","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"cloud","dirs":[{"name":"accessapproval","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"aiplatform","dirs":[{"name":"logging","files":["mod.rs"]},{"name":"v1","dirs":[{"name":"schema","dirs":[{"name":"predict","dirs":[{"name":"instance","files":["mod.rs"]},{"name":"params","files":["mod.rs"]},{"name":"prediction","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"trainingjob","dirs":[{"name":"definition","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"v1beta1","dirs":[{"name":"schema","dirs":[{"name":"predict","dirs":[{"name":"instance","files":["mod.rs"]},{"name":"params","files":["mod.rs"]},{"name":"prediction","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"trainingjob","dirs":[{"name":"definition","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"apigateway","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"apigeeconnect","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"asset","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1p1beta1","files":["mod.rs"]},{"name":"v1p2beta1","files":["mod.rs"]},{"name":"v1p4beta1","files":["mod.rs"]},{"name":"v1p5beta1","files":["mod.rs"]},{"name":"v1p7beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"assuredworkloads","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"audit","files":["mod.rs"]},{"name":"automl","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"baremetalsolution","dirs":[{"name":"v2","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"bigquery","dirs":[{"name":"connection","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"datatransfer","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"logging","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"migration","dirs":[{"name":"v2alpha","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"reservation","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"storage","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]},{"name":"v1beta2","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"v2","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"billing","dirs":[{"name":"budgets","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"binaryauthorization","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"certificatemanager","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"channel","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"clouddms","dirs":[{"name":"logging","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"common","files":["mod.rs"]},{"name":"compute","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1small","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"contactcenterinsights","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"datacatalog","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"datafusion","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"datalabeling","dirs":[{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"dataplex","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"dataproc","dirs":[{"name":"logging","files":["mod.rs"]},{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"dataqna","dirs":[{"name":"v1alpha","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"datastream","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1alpha1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"deploy","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"dialogflow","dirs":[{"name":"cx","dirs":[{"name":"v3","files":["mod.rs"]},{"name":"v3beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"v2","files":["mod.rs"]},{"name":"v2beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"documentai","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]},{"name":"v1beta2","files":["mod.rs"]},{"name":"v1beta3","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"domains","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1alpha2","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"essentialcontacts","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"eventarc","dirs":[{"name":"publishing","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"filestore","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"functions","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"gaming","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"gkebackup","dirs":[{"name":"logging","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"gkeconnect","dirs":[{"name":"gateway","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1alpha1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"gkehub","dirs":[{"name":"cloudauditlogging","dirs":[{"name":"v1alpha","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"configmanagement","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1alpha","files":["mod.rs"]},{"name":"v1beta","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"metering","dirs":[{"name":"v1alpha","files":["mod.rs"]},{"name":"v1beta","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"multiclusteringress","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1alpha","files":["mod.rs"]},{"name":"v1beta","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"servicemesh","dirs":[{"name":"v1alpha","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"v1","files":["mod.rs"]},{"name":"v1alpha","files":["mod.rs"]},{"name":"v1alpha2","files":["mod.rs"]},{"name":"v1beta","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"gsuiteaddons","dirs":[{"name":"logging","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"iap","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"identitytoolkit","dirs":[{"name":"v2","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"ids","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"integrations","dirs":[{"name":"v1alpha","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"iot","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"kms","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"language","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]},{"name":"v1beta2","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"lifesciences","dirs":[{"name":"v2beta","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"location","files":["mod.rs"]},{"name":"managedidentities","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"mediatranslation","dirs":[{"name":"v1alpha1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"memcache","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta2","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"metastore","dirs":[{"name":"logging","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"v1","files":["mod.rs"]},{"name":"v1alpha","files":["mod.rs"]},{"name":"v1beta","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"ml","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"networkconnectivity","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1alpha1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"networkmanagement","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"networksecurity","dirs":[{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"networkservices","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"notebooks","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"orchestration","dirs":[{"name":"airflow","dirs":[{"name":"service","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"orgpolicy","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v2","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"osconfig","dirs":[{"name":"agentendpoint","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"v1","files":["mod.rs"]},{"name":"v1alpha","files":["mod.rs"]},{"name":"v1beta","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"oslogin","dirs":[{"name":"common","files":["mod.rs"]},{"name":"v1","files":["mod.rs"]},{"name":"v1alpha","files":["mod.rs"]},{"name":"v1beta","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"phishingprotection","dirs":[{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"policytroubleshooter","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"privatecatalog","dirs":[{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"pubsublite","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"recaptchaenterprise","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"recommendationengine","dirs":[{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"recommender","dirs":[{"name":"logging","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"redis","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"resourcemanager","dirs":[{"name":"v2","files":["mod.rs"]},{"name":"v3","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"resourcesettings","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"retail","dirs":[{"name":"logging","files":["mod.rs"]},{"name":"v2","files":["mod.rs"]},{"name":"v2alpha","files":["mod.rs"]},{"name":"v2beta","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"runtimeconfig","dirs":[{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"saasaccelerator","dirs":[{"name":"management","dirs":[{"name":"logs","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"scheduler","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"secretmanager","dirs":[{"name":"logging","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"secrets","dirs":[{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"security","dirs":[{"name":"privateca","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"securitycenter","dirs":[{"name":"settings","dirs":[{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]},{"name":"v1p1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"servicedirectory","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"shell","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"speech","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1p1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"sql","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta4","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"support","dirs":[{"name":"common","files":["mod.rs"]},{"name":"v1alpha1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"talent","dirs":[{"name":"v4","files":["mod.rs"]},{"name":"v4beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"tasks","dirs":[{"name":"v2","files":["mod.rs"]},{"name":"v2beta2","files":["mod.rs"]},{"name":"v2beta3","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"texttospeech","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"tpu","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v2alpha1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"translation","dirs":[{"name":"v3","files":["mod.rs"]},{"name":"v3beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"video","dirs":[{"name":"livestream","dirs":[{"name":"logging","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"stitcher","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"transcoder","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"videointelligence","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta2","files":["mod.rs"]},{"name":"v1p1beta1","files":["mod.rs"]},{"name":"v1p2beta1","files":["mod.rs"]},{"name":"v1p3beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"vision","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1p1beta1","files":["mod.rs"]},{"name":"v1p2beta1","files":["mod.rs"]},{"name":"v1p3beta1","files":["mod.rs"]},{"name":"v1p4beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"vmmigration","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"vpcaccess","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"webrisk","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"websecurityscanner","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1alpha","files":["mod.rs"]},{"name":"v1beta","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"workflows","dirs":[{"name":"executions","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"type","files":["mod.rs"]},{"name":"v1","files":["mod.rs"]},{"name":"v1beta","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"container","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1alpha1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"dataflow","dirs":[{"name":"v1beta3","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"datastore","dirs":[{"name":"admin","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"v1","files":["mod.rs"]},{"name":"v1beta3","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"devtools","dirs":[{"name":"artifactregistry","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta2","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"build","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"cloudbuild","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"clouddebugger","dirs":[{"name":"v2","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"clouderrorreporting","dirs":[{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"cloudprofiler","dirs":[{"name":"v2","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"cloudtrace","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v2","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"containeranalysis","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"remoteworkers","dirs":[{"name":"v1test2","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"resultstore","dirs":[{"name":"v2","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"source","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"sourcerepo","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"testing","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"example","dirs":[{"name":"endpointsapis","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"library","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"firebase","dirs":[{"name":"fcm","dirs":[{"name":"connection","dirs":[{"name":"v1alpha1","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"firestore","dirs":[{"name":"admin","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]},{"name":"v1beta2","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"bundle","files":["mod.rs"]},{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"gapic","dirs":[{"name":"metadata","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"genomics","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1alpha2","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"geo","dirs":[{"name":"type","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"home","dirs":[{"name":"enterprise","dirs":[{"name":"sdm","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"graph","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"iam","dirs":[{"name":"admin","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"credentials","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"v1","dirs":[{"name":"logging","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"v1beta","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"identity","dirs":[{"name":"accesscontextmanager","dirs":[{"name":"type","files":["mod.rs"]},{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"logging","dirs":[{"name":"type","files":["mod.rs"]},{"name":"v2","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"longrunning","files":["mod.rs"]},{"name":"maps","dirs":[{"name":"playablelocations","dirs":[{"name":"v3","dirs":[{"name":"sample","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"roads","dirs":[{"name":"v1op","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"routes","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1alpha","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"unity","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"monitoring","dirs":[{"name":"dashboard","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"metricsscope","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"v3","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"networking","dirs":[{"name":"trafficdirector","dirs":[{"name":"type","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"partner","dirs":[{"name":"aistreams","dirs":[{"name":"v1alpha1","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"privacy","dirs":[{"name":"dlp","dirs":[{"name":"v2","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"pubsub","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta2","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"rpc","dirs":[{"name":"context","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"search","dirs":[{"name":"partnerdataingestion","dirs":[{"name":"logging","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"spanner","dirs":[{"name":"admin","dirs":[{"name":"database","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"instance","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"storage","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v2","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"storagetransfer","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"streetview","dirs":[{"name":"publish","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"type","files":["mod.rs"]},{"name":"watcher","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"grafeas","dirs":[{"name":"v1","files":["mod.rs"]},{"name":"v1beta1","dirs":[{"name":"attestation","files":["mod.rs"]},{"name":"build","files":["mod.rs"]},{"name":"deployment","files":["mod.rs"]},{"name":"discovery","files":["mod.rs"]},{"name":"image","files":["mod.rs"]},{"name":"package","files":["mod.rs"]},{"name":"provenance","files":["mod.rs"]},{"name":"source","files":["mod.rs"]},{"name":"vulnerability","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]},{"name":"maps","dirs":[{"name":"fleetengine","dirs":[{"name":"delivery","dirs":[{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"v1","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["mod.rs"]}],"files":["lib.rs"]};
createSourceSidebar();

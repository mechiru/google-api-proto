initSidebarItems({"enum":[["MinimalAction","The minimal action that you want to perform on each instance during the update: - REPLACE: At minimum, delete the instance and create it again. - RESTART: Stop the instance and start it again. - REFRESH: Do not stop the instance. - NONE: Do not disrupt the instance at all. By default, the minimum action is NONE. If your update requires a more disruptive action than you set with this flag, the necessary action is performed to execute the update. Additional supported values which may be not listed in the enum directly due to technical reasons: NONE REFRESH REPLACE RESTART"],["MostDisruptiveAllowedAction","The most disruptive action that you want to perform on each instance during the update: - REPLACE: Delete the instance and create it again. - RESTART: Stop the instance and start it again. - REFRESH: Do not stop the instance. - NONE: Do not disrupt the instance at all. By default, the most disruptive allowed action is REPLACE. If your update requires a more disruptive action than you set with this flag, the update request will fail. Additional supported values which may be not listed in the enum directly due to technical reasons: NONE REFRESH REPLACE RESTART"]]});
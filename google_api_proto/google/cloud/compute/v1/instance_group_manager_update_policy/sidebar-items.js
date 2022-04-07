initSidebarItems({"enum":[["InstanceRedistributionType","The instance redistribution policy for regional managed instance groups. Valid values are: - PROACTIVE (default): The group attempts to maintain an even distribution of VM instances across zones in the region. - NONE: For non-autoscaled groups, proactive redistribution is disabled. Additional supported values which may be not listed in the enum directly due to technical reasons: NONE PROACTIVE"],["MinimalAction","Minimal action to be taken on an instance. You can specify either RESTART to restart existing instances or REPLACE to delete and create new instances from the target template. If you specify a RESTART, the Updater will attempt to perform that action only. However, if the Updater determines that the minimal action you specify is not enough to perform the update, it might perform a more disruptive action. Additional supported values which may be not listed in the enum directly due to technical reasons: NONE REFRESH REPLACE RESTART"],["MostDisruptiveAllowedAction","Most disruptive action that is allowed to be taken on an instance. You can specify either NONE to forbid any actions, REFRESH to allow actions that do not need instance restart, RESTART to allow actions that can be applied without instance replacing or REPLACE to allow all possible actions. If the Updater determines that the minimal update action needed is more disruptive than most disruptive allowed action you specify it will not perform the update at all. Additional supported values which may be not listed in the enum directly due to technical reasons: NONE REFRESH REPLACE RESTART"],["ReplacementMethod","What action should be used to replace instances. See minimal_action.REPLACE"],["Type","The type of update process. You can specify either PROACTIVE so that the instance group manager proactively executes actions in order to bring instances to their target versions or OPPORTUNISTIC so that no action is proactively executed but the update will be performed as part of other actions (for example, resizes or recreateInstances calls). Additional supported values which may be not listed in the enum directly due to technical reasons: PROACTIVE"]]});
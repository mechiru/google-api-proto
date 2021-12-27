initSidebarItems({"enum":[["Type","Types of analysis."]],"mod":[["k_map_estimation_config","Nested message and enum types in `KMapEstimationConfig`."]],"struct":[["CategoricalStatsConfig","Compute numerical stats over an individual column, including number of distinct values and value count distribution."],["DeltaPresenceEstimationConfig","δ-presence metric, used to estimate how likely it is for an attacker to figure out that one given individual appears in a de-identified dataset. Similarly to the k-map metric, we cannot compute δ-presence exactly without knowing the attack dataset, so we use a statistical model instead."],["KAnonymityConfig","k-anonymity metric, used for analysis of reidentification risk."],["KMapEstimationConfig","Reidentifiability metric. This corresponds to a risk model similar to what is called “journalist risk” in the literature, except the attack dataset is statistically modeled instead of being perfectly known. This can be done using publicly available data (like the US Census), or using a custom statistical model (indicated as one or several BigQuery tables), or by extrapolating from the distribution of values in the input dataset."],["LDiversityConfig","l-diversity metric, used for analysis of reidentification risk."],["NumericalStatsConfig","Compute numerical stats over an individual column, including min, max, and quantiles."]]});
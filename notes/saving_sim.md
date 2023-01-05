## Saving

* Save entities and their components.
* Save the following resources:
  * Config
  * SimulationTime;
  * Ecosystem;
  * CountStats;
  * EnergyStats;
  * BugPerformance;
  * FamilyTree;

## Loading

* Load scene data.
* Recreate MindLayout for those entities with a Mind.
* Recreate resources listed above.
* Recreate the following resources from the config.
  * Genome;
  * Spawners;
  * Mind Thresholds;
* Add default resources:
  * LoadedBlueprint;
  * PlantSizeRandomiser;
  * SimulationSpeed;

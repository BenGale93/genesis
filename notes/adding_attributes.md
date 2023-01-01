# Adding an attribute

1. Add the attribute to `genesis_config::attr_config`:
   1. Field of AttributeConfigValidator or DependentAttributeConfigValidator.
   2. Field of AttributeConfig or DependentAttributeConfig.
2. Add the attribute to `genesis_attributes`:
   1. Field of Genome and DNA if the attribute is not dependent.
   2. Add a new attribute component.
   3. Add to the AttributeBundle.
   4. Add to AttributesPlugin.

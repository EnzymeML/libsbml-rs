#pragma once
#include <sbml/SBMLTypes.h>
#include <string>

namespace utils {
    /*
    Model

    These are helper functions for the Model class. Because autocxx does not support
    getting the annotation as a string, we need to use these functions.
    */

    /**
     * Get the annotation of a model as a string.
     * 
     * @param model A pointer to the LIBSBML_CPP_NAMESPACE::Model object.
     * @return A string containing the model's annotation, or an empty string if the model is null or has no annotation set.
     */
    std::string getModelAnnotationString(LIBSBML_CPP_NAMESPACE::Model* model) {
        if (model && model->isSetAnnotation()) {
            return model->getAnnotationString();
        }
        return "";
    }

    /**
     * Set the annotation of a model.
     * 
     * @param model A pointer to the LIBSBML_CPP_NAMESPACE::Model object.
     * @param annotation A string containing the annotation to set for the model.
     */
    void setModelAnnotation(LIBSBML_CPP_NAMESPACE::Model* model, const std::string& annotation) {
        if (model) {
            model->setAnnotation(annotation.c_str());
        }
    }

    /*
    Compartment

    These are helper functions for the Compartment class. Because autocxx does not support
    getting the annotation as a string, we need to use these functions.
    */

    /**
     * Get the annotation of a compartment as a string.
     * 
     * @param compartment A pointer to the LIBSBML_CPP_NAMESPACE::Compartment object.
     * @return A string containing the compartment's annotation, or an empty string if the compartment is null or has no annotation set.
     */
    std::string getCompartmentAnnotationString(LIBSBML_CPP_NAMESPACE::Compartment* compartment) {
        if (compartment && compartment->isSetAnnotation()) {
            return compartment->getAnnotationString();
        }
        return "";
    }

    /**
     * Set the annotation of a compartment.
     * 
     * @param compartment A pointer to the LIBSBML_CPP_NAMESPACE::Compartment object.
     * @param annotation A string containing the annotation to set for the compartment.
     */
    void setCompartmentAnnotation(LIBSBML_CPP_NAMESPACE::Compartment* compartment, const std::string& annotation) {
        if (compartment) {
            compartment->setAnnotation(annotation.c_str());
        }
    }

    /*
    Species

    These are helper functions for the Species class. Because autocxx does not support
    getting the annotation as a string, we need to use these functions.
    */

    /**
     * Get the annotation of a species as a string.
     * 
     * @param species A pointer to the LIBSBML_CPP_NAMESPACE::Species object.
     * @return A string containing the species' annotation, or an empty string if the species is null or has no annotation set.
     */
    std::string getSpeciesAnnotationString(LIBSBML_CPP_NAMESPACE::Species* species) {
        if (species && species->isSetAnnotation()) {
            return species->getAnnotationString();
        }
        return "";
    }

    /**
     * Set the annotation of a species.
     * 
     * @param species A pointer to the LIBSBML_CPP_NAMESPACE::Species object.
     * @param annotation A string containing the annotation to set for the species.
     */
    void setSpeciesAnnotation(LIBSBML_CPP_NAMESPACE::Species* species, const std::string& annotation) {
        if (species) {
            species->setAnnotation(annotation.c_str());
        }
    }

    /*
    UnitDefinition

    These are helper functions for the UnitDefinition class. Because autocxx does not support
    getting the annotation as a string, we need to use these functions.
    */

    /**
     * Get the annotation of a unit definition as a string.
     * 
     * @param unit_definition A pointer to the LIBSBML_CPP_NAMESPACE::UnitDefinition object.
     * @return A string containing the unit definition's annotation, or an empty string if the unit definition is null or has no annotation set.
     */
    std::string getUnitDefinitionAnnotationString(LIBSBML_CPP_NAMESPACE::UnitDefinition* unit_definition) {
        if (unit_definition && unit_definition->isSetAnnotation()) {
            return unit_definition->getAnnotationString();
        }
        return "";
    }


    /**
     * Set the annotation of a unit definition.
     * 
     * @param unit_definition A pointer to the LIBSBML_CPP_NAMESPACE::UnitDefinition object.
     * @param annotation A string containing the annotation to set for the unit definition.
     */
    void setUnitDefinitionAnnotation(LIBSBML_CPP_NAMESPACE::UnitDefinition* unit_definition, const std::string& annotation) {
        if (unit_definition) {
            unit_definition->setAnnotation(annotation.c_str());
        }
    }

    /*
    Unit

    These are helper functions for the Unit class. Because autocxx does not support
    getting the annotation as a string, we need to use these functions.
    */
    
    /**
     * Get the annotation of a unit as a string.
     * 
     * @param unit A pointer to the LIBSBML_CPP_NAMESPACE::Unit object.
     * @return A string containing the unit's annotation, or an empty string if the unit is null or has no annotation set.
     */
    std::string getUnitAnnotationString(LIBSBML_CPP_NAMESPACE::Unit* unit) {
        if (unit && unit->isSetAnnotation()) {
            return unit->getAnnotationString();
        }
        return "";
    }

    /**
     * Set the annotation of a unit.
     * 
     * @param unit A pointer to the LIBSBML_CPP_NAMESPACE::Unit object.
     * @param annotation A string containing the annotation to set for the unit.
     */
    void setUnitAnnotation(LIBSBML_CPP_NAMESPACE::Unit* unit, const std::string& annotation) {
        if (unit) {
            unit->setAnnotation(annotation.c_str());
        }
    }
} 
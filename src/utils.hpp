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
} 
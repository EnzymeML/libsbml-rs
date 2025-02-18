#pragma once
#include "sbml/SBMLTypes.h"
#include <string>

namespace utils {
    std::string getSpeciesAnnotationString(LIBSBML_CPP_NAMESPACE::Species* species) {
        if (species && species->isSetAnnotation()) {
            return species->getAnnotationString();
        }
        return "";
    }
} 
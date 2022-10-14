//
// Created by Osei Fortune on 11/07/2022.
//

#include "Configuration.h"

void Configuration::SetAppBase(std::string base) {
    Configuration::APP_BASE = base;
}

const std::string &Configuration::GetAppBase() {
    return Configuration::APP_BASE;
}

std::string Configuration::APP_BASE = "";
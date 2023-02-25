//
// Created by Osei Fortune on 11/07/2022.
//

#pragma once
#include "Common.h"

class Configuration {
private:
    static std::string APP_BASE;
public:
    static const std::string& GetAppBase();

    static void SetAppBase(std::string base);
};


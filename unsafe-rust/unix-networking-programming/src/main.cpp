#include "../include/easylogging++.h"

INITIALIZE_EASYLOGGINGPP

int main(int argc, char* argv[]) {
    // LOG(GLOBAL) << "This is global";
    LOG(INFO) << "My first info log using default logger";
    LOG(DEBUG) << "This is DEBUG";
    LOG(TRACE) << "This is Trace";
    // LOG(FATAL) << "This is Fatal";
    LOG(ERROR) << "This is Error";
    LOG(WARNING) << "This is warning";
    // LOG(GLOBAL) << "This is global";
    // LOG(GLOBAL) << "This is global";
    return 0;
}
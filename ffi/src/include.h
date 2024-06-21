#pragma once

#include "../PerfUtils/src/TimeTrace.h"

/**
 * This function is the wrapper for TimeTrace::setOutputFileName
 */
inline void timetrace_set_output_filename(const char* filename) {
    PerfUtils::TimeTrace::setOutputFileName(filename);
}

/**
 * This function is the wrapper for TimeTrace::print
 */
inline void timetrace_print() {
    PerfUtils::TimeTrace::print();
}

/**
 * This function is the wrapper for TimeTrace::record with args.
 */
inline void timetrace_record_with_args(const char* format, uint32_t arg0, uint32_t arg1,
                 uint32_t arg2, uint32_t arg3) {
    PerfUtils::TimeTrace::record(format, arg0, arg1, arg2, arg3);
}

/**
 * This function is the wrapper for TimeTrace::record
 */
inline void timetrace_record(const char* format) {
    PerfUtils::TimeTrace::record(format);
}


/**
 * This function is used to set TimeTrace::keepOldEvents
 */
inline void timetrace_set_keepoldevents(bool keep) {
    PerfUtils::TimeTrace::keepOldEvents = keep;
}
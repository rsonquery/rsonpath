#include "Records.h"

extern "C" Record *loadFile(char *file_path);
extern "C" long runJsonSki(char *query, Record *record);
extern "C" void dropFile(Record *record);
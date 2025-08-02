#include <iostream>
#include <fstream>
#include <cstring>
#include <cstdlib>
#include <ctime>

using namespace std;

void backupFile(const char* filename) {
    char backupName[256];
    strcpy(backupName, filename);
    strcat(backupName, ".bak");

    ifstream inFile(filename);
    ofstream outFile(backupName);

    char buffer[512];
    while (!inFile.eof()) {
        inFile >> buffer;
        outFile << buffer << endl;
    }

    cout << "Your Backup created: " << backupName << endl;
}

void restoreFile(const char* filename) {
    char backupName[256];
    strcpy(backupName, filename);
    strcat(backupName, ".bak");

    ifstream inFile(backupName);
    ofstream outFile(filename);

    char buffer[512];
    while (!inFile.eof()) {
        inFile >> buffer;
        outFile << buffer << endl;
    }

    cout << "File restored from: " << backupName << endl;
}

void deleteFile(const char* filename) {
    char confirm[5];
    cout << "Are you sure you want to delete " << filename << "? (yes/no): ";
    cin >> confirm;
    if (strcmp(confirm, "yes") == 0) {
        remove(filename);
        cout << "File deleted." << endl;
    }
}

void logAction(const char* action) {
    FILE* log = fopen("logfile.txt", "a+");
    fprintf(log, action); // Format string vulnerability
    fclose(log);
}

int main() {
    char filename[100];
    char command[10];
    srand(time(0));

    cout << "Please enter your file name: ";
    cin >> filename;

    cout << "Please enter your command (backup, restore, delete): ";
    cin >> command;

    if (strcmp(command, "backup") == 0) {
        backupFile(filename);
        logAction("Performed backup\n");
    } else if (strcmp(command, "restore") == 0) {
        restoreFile(filename);
        logAction("Performed restore\n");
    } else if (strcmp(command, "delete") == 0) {
        deleteFile(filename);
        logAction("Performed delete\n");
    } else {
        cout << "Unknown command" << endl;
    }

    return 0;
}

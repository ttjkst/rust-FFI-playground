#include <sqlite3.h>
#include <printf.h>

//
// Created by ttjkst on 2024/9/13.
//
int main(){
    sqlite3 *db;
    int status = sqlite3_open("/Users/ttjkst/Codes/dir/rust/enlu-db/tests/res/enlu.db",&db);
    printf("%d",status);
    return 0;
}

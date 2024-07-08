/*****************************************************
 *
    curl --request GET \
      --url 'https://api.render.com/v1/services?limit=20' \
      --header 'Accept: application/json' \
      --header 'Authorization: Bearer {{render_api_token_goes_here}}'

*****************************************************************/

#include <cstdlib>
#include <iostream>
#include "./include/dotenv.h"

using namespace std;



// Test
int main() {
    dotenv::init();

    cout << "API KEY : " << getenv("API_KEY") << endl;
}
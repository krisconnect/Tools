#include<stdio.h>
#include<unistd.h>

/*A points to B, C, D
 *B points to A, D
  C points to D
  D points to B, C*/

double l[4][4] = {{0, 0.3333, 0.3333, 0.3333}, {0.5, 0, 0, 0.5}, {0, 0, 0, 1}, {0, 0.5, 0.5, 0}};

double r[2][4] = {{0.25, 0.25, 0.25, 0.25}, {0, 0, 0, 0}};

int main(){

    /* To calculate the rank of a, we need to know three things of all the others:
     * - What is your rank?
     * - Do you link to page "a"?
     * - How many outgoing links do you have in total?*/
    /*   */
    
    int x = 11;
    int m = 0;
    while (x>10){
        double aSum = 0;
        double bSum = 0;
        double cSum = 0;
        double dSum = 0;
            for(int i=0; i<4; i++){
                aSum += r[m][i] * l[i][0];
                bSum += r[m][i] * l[i][1];
                cSum += r[m][i] * l[i][2];
                dSum += r[m][i] * l[i][3];
            }
            printf("%f %f %f %f m is %d here \n", aSum, bSum, cSum, dSum, m);
            
            sleep(1);
            m = (m+1)%2;
            r[m][0] = aSum;
            r[m][1] = bSum;
            r[m][2] = cSum;
            r[m][3] = dSum;
    }
    return 0;
}

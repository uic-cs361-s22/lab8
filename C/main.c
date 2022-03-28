#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <getopt.h>
#include <unistd.h>
#include <pthread.h>
#include <math.h>
#include <sys/time.h>
#include <string.h>
#define MAX_ARRAY_SIZE 1000000000

int threads_count = 1;
int cores_count = 0;
long array_size = 100;
int mode = 0;
long max_cores = 0;
pthread_mutex_t lock;

struct thread_args{
	long start_index;
	long  end_index;
};

long arr[MAX_ARRAY_SIZE]; 
int cores_permitted[100];
long long grand_sum =0;

void mode0(){
	for(long itr=0;itr<array_size;itr++){
		grand_sum+= arr[itr];	
	}
}	
void *mode1(void *args){
	struct thread_args *pargs = (struct thread_args*) args;
	for(long itr=pargs->start_index;itr<pargs->end_index;itr++){
		grand_sum+= arr[itr];	
	}
	free(args);
}	

void *mode2(void *args){
	//To-Do 3: Complete the function body for mode 2
}	
void *mode3(void *args){
	//To-Do 4: Complete the functionm body for mode 3
}	
unsigned int min(long num1,long num2){
	if(num1>num2)
		return num2;
     	return num1;	
}

int cores_extraction(char *string_core){
	int itr=0;
	char *token = strtok(string_core, ",");
	while( token != NULL ) {
		int core = atoi(token)-1;// Subtracting -1 to compensate 0 indexing
		if(core>max_cores){
			printf("Requested core not within range\n");
			return 0;
		}
		cores_permitted[itr]=core;
		token = strtok(NULL, ",");
		itr++;
	}
	return itr;
}

int main(int argc, char **argv){
	int option;
	max_cores = sysconf( _SC_NPROCESSORS_ONLN );
	while((option = getopt(argc,argv, "ht:c:n:m:")) != -1){
		switch(option){
			case 'h': printf("Options:\n");
				  printf("t: Threads Count\n");
				  printf("c: Cores Count\n");
				  printf("n: Array Size\n");
				  printf("m: Mode\n");
				  printf("   Mode 0: 1 thread no lock\n");
				  printf("   Mode 1: Multi-threads no lock\n");
				  printf("   Mode 2: Multi-threads with lock\n");
				  printf("   Mode 3: Multi-threads with lock and grouped sum\n");
				  return 0;
			case 't': threads_count = atoi(optarg);
				  break;
			case 'c': cores_count = cores_extraction(optarg);
				  break;
			case 'n': array_size = atol(optarg);
				  if(array_size >= MAX_ARRAY_SIZE){
				  	printf("Array size cannot be greater than %d\n",MAX_ARRAY_SIZE-1);
					return 0;
				  }
				  break;
			case 'm': mode = atoi(optarg);
				  if(mode<0 || mode >3){
				  	printf("Invalid mode, check out help with -h option\n");
					return 0;
				  }
				  break;
			default: printf("Unknown option, look out for the help\n");
		}
	}
	
	//Set Core Affinity
	cpu_set_t cpuset;
	pthread_t thread;
	thread = pthread_self();
	CPU_ZERO(&cpuset);
	if(cores_count==0){
		CPU_SET(0, &cpuset);
		cores_count = 1;
	}
	else{
		for(int core_itr=0;core_itr<cores_count;core_itr++){
			CPU_SET(cores_permitted[core_itr], &cpuset);
		}
	}
	int status = pthread_setaffinity_np(thread, sizeof(cpuset), &cpuset);
	if(status!=0){
		printf("Issue is Assigning Cores");
		return 0;
	}

	//Print Configuration
	printf("\n# CONFIGURATION #\n");
	printf("Threads Count: %d\n", threads_count);
	printf("Cores Count: %d\n", cores_count);
	printf("Array Size: %ld\n", array_size);
	printf("Mode : %d\n", mode);
	
	//Populate the Array,
	//Array Population can be made multithreaded, 
	//Can populate random numbers of size int
	printf("\nPopulating the Array...\n");
	for(long itr=0;itr<array_size;itr++){
		arr[itr]=itr;
	}	
	
	struct timeval tval_before, tval_after, tval_result;
	printf("Starting the Experiment...\n");
	gettimeofday(&tval_before, NULL);
 	
	if(mode == 0){
		mode0();
	}
	else 
	{
		pthread_t threadpool[threads_count];
		int slice_size = ceil(array_size * 1.0/ threads_count);
		//Create Worker Threads
		for(int itr1 = 0;itr1< threads_count; itr1++){
			struct thread_args *args = (struct thread_args*) malloc(sizeof(struct thread_args));
			args->start_index = itr1*slice_size;
			args->end_index =  min(args->start_index + slice_size,array_size);
			printf("Thread %d will cover from %ld to %ld\n",itr1,args->start_index,args->end_index)  ;
			//To-Do 1: Spawn threads for various modes, pass the appropriate thread function
			switch(mode){
				case 1: //Create Thread for Mode 1 here
					break;
				case 2: //Create Thread for Mode 2 here
					break;
				case 3: //Create Thread for Mode 3 here
					break;
			}
		}
 		//Join Threads	
		for(int itr2=0;itr2<threads_count;itr2++){
			//To-Do 2: Wait for threads to finish their jobs
		}
	}
	gettimeofday(&tval_after, NULL);
	timersub(&tval_after, &tval_before, &tval_result);
	printf("\n# RESULT #\n");
	printf("Time elapsed: %ld.%06ld\n", (long int)tval_result.tv_sec, (long int)tval_result.tv_usec);
	printf("Grand Sum:%lld\n",grand_sum);
	
	return 0;
}

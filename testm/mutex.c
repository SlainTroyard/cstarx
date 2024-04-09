
#include <pthread.h>
#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <semaphore.h>
 
void *thread1_function(void *arg);
void *thread2_function(void *arg);
pthread_mutex_t mutex;
 
int main(void)
{
	pthread_t pthread1, pthread2;
	int ret;
 
	pthread_mutex_init(&mutex, NULL);
 
	ret = pthread_create(&pthread1, NULL, thread1_function, NULL);
	if(ret != 0)
	{
		perror("pthread_create");
		exit(1);
	}
 
	ret = pthread_create(&pthread2, NULL, thread2_function, NULL);
	if(ret != 0)
	{
		perror("pthread_create");
		exit(1);
	}
 
	pthread_join(pthread1, NULL);
	pthread_join(pthread2, NULL);
	printf("The thread is over, process is over too.\n");
 
	return 0;
}
 
void *thread1_function(void *arg)
{
	int i;
 
	while(1)
	{
		pthread_mutex_lock(&mutex);
		for(i = 0; i < 10; i++)
		{
			printf("Hello world\n");
			sleep(1);
		}
		pthread_mutex_unlock(&mutex);
		sleep(1);
	}
	return NULL;
}
 
void *thread2_function(void *arg)
{
	int i;
	sleep(1);
 
	while(1)
	{
		pthread_mutex_lock(&mutex);
		for(i = 0; i < 10; i++)
		{
			printf("Good moring\n");
			sleep(1);
		}
		pthread_mutex_unlock(&mutex);
		sleep(1);
	}
	return NULL;
}
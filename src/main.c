
/*
 * This file is needed, because elf_getdata() works when called from C code, but not from rust code.
 *
 *
 *
 */

#include <err.h>
#include <fcntl.h>
#include <libelf.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#include <string.h>

void test(int fd_from_rust){

	Elf *e;
	Elf_Scn * scn;

	//FILE* fd = fopen("/home/me/work/tori-victorinix/victorinix", "r");
	int fd = open("/home/me/work/tori-victorinix/victorinix", O_RDONLY , 0);

	printf("hiiiiiiii from C: %d \n", fd);
	//printf("hiiiiiiii from C: %d \n", fd_from_rust);
	//if (fd < 0)
		//errx(EXIT_FAILURE , "open failed");

	return;
	e = elf_begin(3, ELF_C_RDWR, NULL);
	printf("elf from C: %d \n", e);
	if (e == NULL)
		errx(EXIT_FAILURE , "elf_begin ()␣failed:␣%s.", elf_errmsg ( -1));

	if ((scn = elf_getscn(e, 29)) == NULL)
		errx(EXIT_FAILURE , "getscn failed from C: ", elf_errmsg ( -1));

	printf("scn from C: 0x%x\n", scn);
}

struct MyGetdataCReturn {
	unsigned char * data_ptr;
	int data_len;
	unsigned char * err_ptr;
};

struct MyGetdataCReturn my_getdata_c(Elf* elf, int props_section) {

	struct MyGetdataCReturn to_return;

	Elf_Data * data;
	Elf_Scn * scn;

	printf("hello from C to\n");

	if ((scn = elf_getscn(elf, 29)) == NULL) {
		printf("elf_getscn failed");
		to_return.data_ptr = 0;
		to_return.data_len = 0;
		unsigned char error_string [100] = "elf_getscn failed";
	}

	//printf("scn: 0x%x\n", scn);
	//printf("scn number: 0x%x\n", scn->s_index);
	//dump((void*) scn, 10);

	data = elf_getdata ( scn , data );
	//if (( data = elf_getdata ( scn , data )) == NULL ) {
	//}

	unsigned char error_string [15] = "hello world";

	printf("data: %x\n", data);
	printf("d_buf: %x\n", data->d_buf);
	to_return.data_ptr = (unsigned char *) data->d_buf;
	printf("hi 2\n");
	to_return.data_len = data->d_size;
	to_return.err_ptr = &error_string;
}



int dump(void* data, int len) {

	int value = 10;
	int hi = &value;
	printf("addr: %x\n", hi);

	unsigned char* bytes = (unsigned char* ) &data;

	//char* bytes = (char*) data;

	//arrays to map hex to bin
	char hex_chars[17] = {'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'};
	char bin_chars[17][4] = {"0000", "0001", "0010", "0011", "0100", "0101", "0110", "0111", "1000", "1001", "1010", "1011", "1100", "1101", "1110", "1111"};

	//memcpy(bytes, &number, sizeof(number));

	printf("Address of bytes: 0x%x\n", bytes);
	printf("len of bytes: %d\n", len);


	for (int i = 0; i < len; i++) {
		printf("%x ... 0x%02x \n", bytes+i, *(bytes+i));
	}
	printf("dump done\n");

	// line before
	printf("========");
	for (int i = 0; i < len; i++) {
		printf("===============");
	}
	printf("\n");

	//Print hex line
	printf("  HEX  |  ");
	//for (int i = len -1; i >= 0; i-=1) { // absteigend
	for (int i = 0; i < len; i++) { // aufsteigend
		printf("  0x%02x      |  ", *(bytes+i));
		//printf("i: %d ...  0x%02x \n", i, *(bytes+i));
		//printf("i: %d ...  \n", i);
	}
	printf("\n");

	//Print bin line
	 /*
	printf("  BIN  |  ");
	for (int i = len -1; i >= 0; i-=1) {

		char temp[3];
		sprintf(temp, "%02X", *(bytes+i));

		for (int h = 0; h < 2; h++) {

			for (int g = 0; g < 16; g++) {
				if (temp[h] == hex_chars[g]){
					for (int j = 0; j < 4; j++) {
						printf("%c", bin_chars[g][j]);
					}
					printf("  ");
					// printf("  %s}\n", bin_chars[g]);
				}
			}
		}
		printf("|  ");


	}
	printf("\n");
	// */

	// print line of = after data
	printf("========");
	for (int i = 0; i < len; i++) {
		printf("===============");
	}
	printf("\n\n");


	return 0;

}

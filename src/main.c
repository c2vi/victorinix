
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
#include <gelf.h>
#include <stdint.h>

struct GetPropsReturn {
	unsigned char * data_ptr;
	unsigned char * err_ptr;
};

int ERR_MSG_LEN = 200;
static char VICTORINIX_SECTION_NAME [20] = ".victorinix_props";
static char err_msg [200] = "";
static Elf *elf;
static int fd;

char ** get_props_c(char * path) {

	printf("hi from c\n");
	my_errmsg("hiiiiiiiiiiiiiiiiiii: ", elf_errmsg(-1));
	printf("hi from c\n");
	printf("err: %s\n", err_msg);
	Elf_Scn * scn;
	size_t shstrndx;

	if (elf_version(EV_CURRENT) == EV_NONE) {
		my_errmsg("error in elf version: ", elf_errmsg(-1));
		printf("err: %s\n", err_msg);
		return;
	}

	fd = open("/home/me/work/tori-victorinix/victorinix", O_RDWR , 0);

	elf = elf_begin(fd, ELF_C_RDWR, NULL);
	if (elf == NULL) {
		my_errmsg("elf_begin() failed: ", elf_errmsg(-1));
		printf("err: %s\n", err_msg);
		return;
	}

	int victorinix_section = get_victorinix_section();
	if (victorinix_section == 0) {
		printf("err: %s\n", err_msg);
		return;
	}
	printf("vic section at: %d\n", victorinix_section);

	GElf_Shdr shdr;
	scn = elf_getscn(elf, victorinix_section);

	gelf_getshdr(scn, &shdr);

	char * section_name = elf_strptr(elf, shstrndx, shdr.sh_name);

	printf("vic section_name: %s\n", section_name);


	my_errmsg("not finished", 0);
	char tmp [4] = "hi";
	char * hi [2] = {err_msg, &tmp};
	return hi;
}


int get_victorinix_section() {

	Elf_Scn * scn = NULL;
	GElf_Shdr shdr;
	size_t shstrndx;
	char * section_name;
	int counter = 1;

	if ( elf_getshdrstrndx (elf, &shstrndx ) != 0) {
		my_errmsg("elf_getshdrstrndx() faled: ", elf_errmsg(-1));
		return;
	}

	while (( scn = elf_nextscn(elf, scn)) != NULL ) {
		if ( gelf_getshdr(scn, &shdr) != &shdr ) {
			my_errmsg("elf_getshdr() failed: ", elf_errmsg(-1));
			return 0;
		}

		section_name = elf_strptr(elf, shstrndx, shdr.sh_name);
		if (section_name == NULL) {
			my_errmsg("elf_strptr() failed: ", elf_errmsg(-1));
			return 0;
		}

		if (strcmp(section_name, &VICTORINIX_SECTION_NAME)) {
			return counter;
		}
		counter++;
	}

}

void my_errmsg(char * err_msg_one, char * err_msg_two) {
	printf("my_errmsg one: %s\n", err_msg_one);
	printf("my_errmsg two: %s\n", err_msg_two);
	strncpy(err_msg, err_msg_one, ERR_MSG_LEN -1);
	if (err_msg_two != 0) {
		int len = ERR_MSG_LEN -1 -(strlen(err_msg));
		strncpy(err_msg + strlen(err_msg_one), err_msg_two, len);
	}
}

void drop_elf(){
	elf_end(elf);
	close(fd);
}


void test_old(int fd_from_rust){

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
	printf("addr: %p\n", &value);

	unsigned char* bytes = (unsigned char* ) &data;

	//char* bytes = (char*) data;

	//arrays to map hex to bin
	char hex_chars[17] = {'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'};
	char bin_chars[17][4] = {"0000", "0001", "0010", "0011", "0100", "0101", "0110", "0111", "1000", "1001", "1010", "1011", "1100", "1101", "1110", "1111"};

	//memcpy(bytes, &number, sizeof(number));

	printf("Address of bytes: 0x%p\n", bytes);
	printf("len of bytes: %d\n", len);


	for (int i = 0; i < len; i++) {
		printf("%p ... 0x%02x \n", bytes+i, *(bytes+i));
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

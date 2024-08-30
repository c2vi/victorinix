{
	disks = {
		default = {
			kind = "gpt";
			# kind = "mbr";
			partitions = [ "3" "5" ];
		};
	};
	items = {
		"3" = {
			kind = "fat32";
		};
	};

}
{
  disks.default = {
    gpt = { /* gpt settings */ };
    gpt.partitions = [
      {
        size = "200M";
        fat32 = true;
      }
      {
        luks = true;
      }
    ];
    mbt = { /* mbr settrings*/ };
    content = "hello world";
  };
}

rm -rf testmkdir
rm -rf testsym

mkdir testmkdir
cd testmkdir || exit
touch test1.mp4
touch test2.mkv
touch test3.avi
touch test4.txt
cd ..

mkdir testsym
ln -s ../testmkdir/test1.mp4 testsym
ln -s ../testmkdir/test2.mkv testsym
ln -s ../testmkdir/test3.avi testsym
ln -s ../testmkdir/test4.txt testsym
ln -s ../testmkdir/test.fail testsym
rm -rf testmkdir
rm -rf testsym
rm -rf testrn

mkdir testmkdir
cd testmkdir || exit
touch test1.mp4
touch test2.mkv
touch test3.avi
touch test4.txt
mkdir recursive
cp test* recursive
cd ..

mkdir testsym
mkdir testsym/recursive
ln -s ../testmkdir/test1.mp4 testsym
ln -s ../testmkdir/test2.mkv testsym
ln -s ../testmkdir/test3.avi testsym
ln -s ../testmkdir/test4.txt testsym
ln -s ../testmkdir/test.fail testsym

ln -s ../../testmkdir/test1.mp4 testsym/recursive
ln -s ../../testmkdir/test2.mkv testsym/recursive
ln -s ../../testmkdir/test3.avi testsym/recursive
ln -s ../../testmkdir/test4.txt testsym/recursive
ln -s ../../testmkdir/test.fail testsym/recursive

mkdir testrn
mkdir testrn/recursive
touch testrn/abcTEST1.txt
touch testrn/TabcEST2.txt
touch testrn/TEabcST3.txt
touch testrn/TESabcT4.txt
touch testrn/TESTabc5.txt
touch testrn/recursive/abcTEST1.txt
touch testrn/recursive/TabcEST2.txt
touch testrn/recursive/TEabcST3.txt
touch testrn/recursive/TESabcT4.txt
touch testrn/recursive/TESTabc5.txt
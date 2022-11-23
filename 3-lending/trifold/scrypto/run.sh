resim publish . > publish.log
resim call-function `cat publish.log | tail -c 55` Trifold instantiate > instantiate.log
export COMPONENT=`cat instantiate.log | grep Component: | tail -c 55`
echo $COMPONENT
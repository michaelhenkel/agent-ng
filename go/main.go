package main

import (
	"context"
	"io"
	"log"
	"os"

	pbv1 "github.com/michaelhenkel/config-controller/pkg/apis/v1"
	"google.golang.org/grpc"
	"k8s.io/klog/v2"
	contrail "ssd-git.juniper.net/contrail/cn2/contrail/pkg/apis/core/v1alpha1"
)

var handlerMap map[string]ResourceHandler

type ResourceHandler interface {
	Handle(pbv1.ConfigControllerClient, pbv1.Resource) error
}

type VirtualNetwork struct {
	*contrail.VirtualNetwork
}

func (r *VirtualNetwork) Handle(client pbv1.ConfigControllerClient, resource pbv1.Resource) error {
	res, err := client.GetVirtualNetwork(context.Background(), &resource)
	if err != nil {
		return err
	}
	klog.Infof("got resource %s %s/%s", res.Kind, res.Namespace, res.Name)
	return nil
}

func main() {
	handlerMap = make(map[string]ResourceHandler)
	handlerMap["VirtualNetwork"] = &VirtualNetwork{}
	nodeName := "5b3s30"
	if len(os.Args) > 1 {
		nodeName = os.Args[1]
	}
	controllerAddress := "127.0.0.1:20443"
	var opts []grpc.DialOption
	opts = append(opts, grpc.WithBlock())
	opts = append(opts, grpc.WithInsecure())
	conn, err := grpc.Dial(controllerAddress, opts...)
	if err != nil {
		log.Fatalf("fail to dial: %v", err)
	}
	defer conn.Close()
	var done = make(chan bool)
	client := pbv1.NewConfigControllerClient(conn)
	go subscribe(client, nodeName)
	<-done
}

func subscribe(client pbv1.ConfigControllerClient, nodeName string) {
	req := &pbv1.SubscriptionRequest{
		Name: nodeName,
	}
	stream, err := client.SubscribeListWatch(context.Background(), req)
	if err != nil {
		klog.Fatal(err)
	}
	for {
		resource, err := stream.Recv()
		if err == io.EOF {
			break
		}
		if err != nil {
			log.Fatalf("%v.req(_) = _, %v", client, err)
		}
		if handler, ok := handlerMap[resource.Kind]; ok {
			if err := handler.Handle(client, *resource); err != nil {
				klog.Error(err)
			}
		}
	}
}

/*
func update(response *pbv1.Response) error {
	switch t := response.New.Resource.(type) {
	case *pbv1.Resource_VirtualNetwork:
		klog.Infof("got vn update: new %s old %s", t.VirtualNetwork.Name, response.Old.GetVirtualNetwork().Name)
	case *pbv1.Resource_VirtualMachineInterface:
		klog.Infof("got vmi update: new %s old %s", t.VirtualMachineInterface.Name, response.Old.GetVirtualMachineInterface().Name)
	case *pbv1.Resource_VirtualRouter:
		klog.Infof("got vrouter update: new %s old %s", t.VirtualRouter.Name, response.Old.GetVirtualRouter().Name)
	}
	return nil
}

func add(response *pbv1.Response) {
	switch t := response.New.Resource.(type) {
	case *pbv1.Resource_VirtualNetwork:
		klog.Infof("got vn %s add", t.VirtualNetwork.Name)
	case *pbv1.Resource_VirtualMachineInterface:
		klog.Infof("got vmi %s add", t.VirtualMachineInterface.Name)
	case *pbv1.Resource_VirtualRouter:
		klog.Infof("got vrouter %s add", t.VirtualRouter.Name)
	case *pbv1.Resource_VirtualMachine:
		klog.Infof("got vm %s add", t.VirtualMachine.Name)
	case *pbv1.Resource_RoutingInstance:
		klog.Infof("got ri %s add", t.RoutingInstance.Name)
	case *pbv1.Resource_InstanceIP:
		klog.Infof("got iip %s add", t.InstanceIP.Name)
	default:
		klog.Infof("got unknown %s add", t)
	}
}
*/

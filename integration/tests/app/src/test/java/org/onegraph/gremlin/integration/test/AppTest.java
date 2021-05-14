/*
 * This Java source file was generated by the Gradle 'init' task.
 */
package org.onegraph.gremlin.integration.test;

import java.util.List;

import org.apache.tinkerpop.gremlin.driver.Cluster;
import org.apache.tinkerpop.gremlin.driver.remote.DriverRemoteConnection;
import org.apache.tinkerpop.gremlin.driver.ser.Serializers;
import org.apache.tinkerpop.gremlin.process.traversal.AnonymousTraversalSource;
import org.apache.tinkerpop.gremlin.process.traversal.P;
import org.apache.tinkerpop.gremlin.process.traversal.dsl.graph.GraphTraversalSource;
import org.apache.tinkerpop.gremlin.structure.Vertex;
import org.junit.Test;

public class AppTest {
    @Test 
    public void testAppHasAGreeting() {

        final Cluster cluster = createCluster();
        try {
            final GraphTraversalSource g = AnonymousTraversalSource.traversal().withRemote(DriverRemoteConnection.using(cluster));
            List<Object> verticesWithNamePumba = g.V().has("name", "pumba").out("friendOf").id().toList();
            System.out.println(verticesWithNamePumba);
        } finally {
            cluster.close();
        }

    }

    @Test
    public void testMatchEdge() {
        final Cluster cluster = createCluster();
        try {
            final GraphTraversalSource g = AnonymousTraversalSource.traversal().withRemote(DriverRemoteConnection.using(cluster));
            var v1 = g.V(1).as("source").outE("hasFriend", "contains").V().as("target").V().addE("testEdge").to("target").iterate();
            System.out.println(v1);
        } finally {
            cluster.close();
        }
    }

    @Test
    public void testCreateVertex() {
        final Cluster cluster = createCluster();
        try {
            final GraphTraversalSource g = AnonymousTraversalSource.traversal().withRemote(DriverRemoteConnection.using(cluster));
            var v1 = g.addV("person").property("name","marko").iterate();
            Vertex v2 = g.addV("person").property("name","stephen").next();
            g.V(v1).addE("knows").to(v2).property("weight",0.75).iterate();
            System.out.println(v1);
        } finally {
            cluster.close();
        }
    }

    @Test
    public void testCreateEdge() {
        final Cluster cluster = createCluster();
        try {
            final GraphTraversalSource g = AnonymousTraversalSource.traversal().withRemote(DriverRemoteConnection.using(cluster));
            var v1 = g.V().has("name", P.within("marko", "stephen")).as("person").
            V().has("name", P.within("stephen")).addE("uses").from("person").next();
            System.out.println(v1);
        } finally {
            cluster.close();
        }
    }
    

    private Cluster createCluster() {
        final Cluster cluster = Cluster.build("localhost")
        .port(8182)
        .maxInProcessPerConnection(32)
        .maxSimultaneousUsagePerConnection(32)
        .serializer(Serializers.GRAPHSON_V3D0)
        .create();
        return cluster;
    }
}

